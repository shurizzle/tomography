#![allow(non_camel_case_types)]

use crate::types::{fs::*, Error, Result};
use errno::errno;
use libc::{statfs, sysctl, CTL_VFS};
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_long, c_void};
use std::path::PathBuf;
use std::ptr::null_mut;

use core_foundation::base::{TCFType, TCFTypeRef};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::uuid::{CFUUIDRef, CFUUID};

use uuid::Uuid;

use super::disk_arbitration::{
    disk::{kDADiskDescriptionVolumeNameKey, kDADiskDescriptionVolumeUUIDKey},
    DADisk, DASession,
};

const VFS_GENERIC: c_int = 0;

const MFSNAMELEN: usize = 15;

const VFS_MAXTYPENUM: c_int = 1;
const VFS_CONF: c_int = 2;

const MNT_NOWAIT: c_int = 2;

#[repr(C)]
struct vfsconf {
    vfc_reserved: u32,
    vfc_name: [c_char; MFSNAMELEN],
    vfc_typenum: c_int,
    vfc_refcount: c_int,
    vfc_flags: c_int,
    vfc_reserved2: u32,
    vfc_reserved3: u32,
}

extern "C" {
    #[link_name = "getfsstat$INODE64"]
    fn getfsstat(mntbufp: *mut statfs, bufsize: c_long, mode: c_int) -> c_int;
}

macro_rules! c_str_to_string {
    ($x:expr) => {
        String::from(unsafe { CStr::from_ptr($x.as_ptr()).to_str().unwrap().trim() })
    };
}

pub fn fs_names() -> Result<Vec<String>> {
    let mut mib: [c_int; 4] = unsafe { std::mem::zeroed() };
    let mut maxvfsconf: c_int = 0;
    let mut miblen: usize = std::mem::size_of::<c_int>();

    mib[0] = CTL_VFS;
    mib[1] = VFS_GENERIC;
    mib[2] = VFS_MAXTYPENUM;

    if unsafe {
        sysctl(
            mib.as_mut_ptr(),
            3,
            &mut maxvfsconf as *mut c_int as *mut c_void,
            &mut miblen,
            null_mut(),
            0,
        )
    } != 0
    {
        return Err(Error::from_errno());
    }

    miblen = std::mem::size_of::<vfsconf>();
    mib[2] = VFS_CONF;
    let mut res: Vec<String> = Vec::with_capacity(maxvfsconf as usize);
    for i in 0..maxvfsconf {
        mib[3] = i;
        let mut vfc: vfsconf = unsafe { std::mem::zeroed() };

        if unsafe {
            sysctl(
                mib.as_mut_ptr(),
                4,
                &mut vfc as *mut vfsconf as *mut c_void,
                &mut miblen,
                null_mut(),
                0,
            )
        } != 0
        {
            if errno().0 == libc::ENOTSUP {
                continue;
            }
            return Err(Error::from_errno());
        }

        res.push(c_str_to_string!(vfc.vfc_name));
    }

    Ok(res)
}

fn fs_stat() -> Result<Vec<statfs>> {
    let mut tries: usize = 1;
    let mut mntsize = unsafe { getfsstat(null_mut(), 0, MNT_NOWAIT) };
    if mntsize < 0 {
        return Err(Error::from_errno());
    }

    let mut mntbuf: Vec<statfs> = Vec::with_capacity((mntsize as usize) * 2);
    let mut bufsize: usize = (mntsize as usize) * 2 * std::mem::size_of::<statfs>();

    mntsize = unsafe { getfsstat(mntbuf.as_mut_ptr(), bufsize as c_long, MNT_NOWAIT) };
    if mntsize < 0 {
        return Err(Error::from_errno());
    }

    unsafe {
        mntbuf.set_len(if mntbuf.capacity() < (mntsize as usize) {
            mntbuf.capacity()
        } else {
            mntsize as usize
        });
    }

    while tries < 3 && bufsize <= ((mntsize as usize) * std::mem::size_of::<statfs>()) {
        tries += 1;
        mntbuf = Vec::with_capacity((mntsize as usize) * 2);
        bufsize = (mntsize as usize) * 2 * std::mem::size_of::<statfs>();
        mntsize = unsafe { getfsstat(mntbuf.as_mut_ptr(), bufsize as c_long, MNT_NOWAIT) };
        if mntsize < 0 {
            return Err(Error::from_errno());
        }

        unsafe {
            mntbuf.set_len(if mntbuf.capacity() < (mntsize as usize) {
                mntbuf.capacity()
            } else {
                mntsize as usize
            });
        }
    }

    Ok(mntbuf)
}

pub fn all() -> Result<Vec<FileSystem>> {
    let fss = fs_names()?;
    let session = DASession::new();

    let mut res: Vec<FileSystem> = fs_stat()?
        .iter()
        .filter_map(|x| {
            let device = PathBuf::from(c_str_to_string!(x.f_mntfromname));
            let disk = DADisk::from_path(&session, &device)?;
            let props = disk.description()?;
            let uuid: String = unsafe {
                props
                    .find(&CFString::wrap_under_get_rule(
                        kDADiskDescriptionVolumeUUIDKey,
                    ))
                    .map(|v| {
                        format!(
                            "{}",
                            Into::<Uuid>::into(CFUUID::wrap_under_get_rule(
                                CFUUIDRef::from_void_ptr(*v)
                            ))
                        )
                    })?
            };
            let label: String = unsafe {
                props
                    .find(&CFString::wrap_under_get_rule(
                        kDADiskDescriptionVolumeNameKey,
                    ))
                    .map(|v| {
                        CFString::wrap_under_get_rule(CFStringRef::from_void_ptr(*v)).to_string()
                    })?
            };

            Some(FileSystem {
                device,
                filesystem: c_str_to_string!(x.f_fstypename),
                mountpoint: PathBuf::from(c_str_to_string!(x.f_mntonname)),
                uuid,
                label,
                total: x.f_blocks * (x.f_bsize as u64),
                free: x.f_bfree * (x.f_bsize as u64),
                used: (x.f_blocks - x.f_bfree) * (x.f_bsize as u64),
            })
        })
        .filter(|x| fss.contains(&x.filesystem))
        .collect();

    res.sort_by(|a, b| b.mountpoint.partial_cmp(&a.mountpoint).unwrap());

    Ok(res)
}

#[cfg(test)]
mod test {
    #[test]
    fn it_werks() {
        println!("{:#?}", super::all());
    }
}
