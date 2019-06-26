use libc::{c_void, memcpy, sysctl, timespec, timeval, CTL_KERN, KERN_BOOTTIME};
use std::time::SystemTime;

pub fn get() -> Option<SystemTime> {
    let mut res: timeval = unsafe { std::mem::zeroed() };
    let mut len = std::mem::size_of::<timeval>();
    let mut mib = [CTL_KERN, KERN_BOOTTIME];
    let err = unsafe {
        sysctl(
            &mut mib[0],
            2,
            &mut res as *mut _ as *mut c_void,
            &mut len,
            std::ptr::null_mut(),
            0,
        )
    };
    if err < 0 {
        None
    } else {
        let mut ts: timespec = unsafe { std::mem::zeroed() };
        ts.tv_sec = res.tv_sec;
        ts.tv_nsec = i64::from(res.tv_usec) * 1_000;
        let mut time = SystemTime::now();
        unsafe {
            memcpy(
                &mut time as *mut _ as *mut c_void,
                &mut ts as *mut _ as *mut c_void,
                std::mem::size_of::<timespec>(),
            )
        };
        Some(time)
    }
}
