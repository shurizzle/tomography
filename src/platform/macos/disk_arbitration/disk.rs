#![allow(dead_code)]

use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::path::Path;

use core_foundation::base::{kCFAllocatorDefault, CFAllocatorRef, CFTypeID, TCFType};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::url::{CFURLRef, CFURL};
use core_foundation::{declare_TCFType, impl_TCFType};

use super::session::{DASession, DASessionRef};

#[repr(C)]
pub struct __DADisk(c_void);
pub type DADiskRef = *mut __DADisk;

#[link(name = "DiskArbitration", kind = "framework")]
extern "C" {
    fn DADiskGetTypeID() -> CFTypeID;
    fn DADiskCreateFromVolumePath(
        allocator: CFAllocatorRef,
        session: DASessionRef,
        path: CFURLRef,
    ) -> DADiskRef;
    fn DADiskCreateFromBSDName(
        allocator: CFAllocatorRef,
        session: DASessionRef,
        name: *mut c_char,
    ) -> DADiskRef;
    fn DADiskCopyDescription(disk: DADiskRef) -> CFDictionaryRef;

    pub static kDADiskDescriptionVolumeKindKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionVolumeMountableKey: CFStringRef; /* ( CFBoolean    ) */
    pub static kDADiskDescriptionVolumeNameKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionVolumeNetworkKey: CFStringRef; /* ( CFBoolean    ) */
    pub static kDADiskDescriptionVolumePathKey: CFStringRef; /* ( CFURL        ) */
    pub static kDADiskDescriptionVolumeTypeKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionVolumeUUIDKey: CFStringRef; /* ( CFUUID       ) */

    pub static kDADiskDescriptionMediaBlockSizeKey: CFStringRef; /* ( CFNumber     ) */
    pub static kDADiskDescriptionMediaBSDMajorKey: CFStringRef; /* ( CFNumber     ) */
    pub static kDADiskDescriptionMediaBSDMinorKey: CFStringRef; /* ( CFNumber     ) */
    pub static kDADiskDescriptionMediaBSDNameKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionMediaBSDUnitKey: CFStringRef; /* ( CFNumber     ) */
    pub static kDADiskDescriptionMediaContentKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionMediaEjectableKey: CFStringRef; /* ( CFBoolean    ) */
    pub static kDADiskDescriptionMediaIconKey: CFStringRef; /* ( CFDictionary ) */
    pub static kDADiskDescriptionMediaKindKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionMediaLeafKey: CFStringRef; /* ( CFBoolean    ) */
    pub static kDADiskDescriptionMediaNameKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionMediaPathKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionMediaRemovableKey: CFStringRef; /* ( CFBoolean    ) */
    pub static kDADiskDescriptionMediaSizeKey: CFStringRef; /* ( CFNumber     ) */
    pub static kDADiskDescriptionMediaTypeKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionMediaUUIDKey: CFStringRef; /* ( CFUUID       ) */
    pub static kDADiskDescriptionMediaWholeKey: CFStringRef; /* ( CFBoolean    ) */
    pub static kDADiskDescriptionMediaWritableKey: CFStringRef; /* ( CFBoolean    ) */
    pub static kDADiskDescriptionMediaEncryptedKey: CFStringRef; /* ( CFBoolean    ) */
    pub static kDADiskDescriptionMediaEncryptionDetailKey: CFStringRef; /* ( CFNumber ) */

    pub static kDADiskDescriptionDeviceGUIDKey: CFStringRef; /* ( CFData       ) */
    pub static kDADiskDescriptionDeviceInternalKey: CFStringRef; /* ( CFBoolean    ) */
    pub static kDADiskDescriptionDeviceModelKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionDevicePathKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionDeviceProtocolKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionDeviceRevisionKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionDeviceUnitKey: CFStringRef; /* ( CFNumber     ) */
    pub static kDADiskDescriptionDeviceVendorKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionDeviceTDMLockedKey: CFStringRef; /* ( CFBoolean    ) */

    pub static kDADiskDescriptionBusNameKey: CFStringRef; /* ( CFString     ) */
    pub static kDADiskDescriptionBusPathKey: CFStringRef; /* ( CFString     ) */
}

declare_TCFType! {DADisk, DADiskRef}
impl_TCFType!(DADisk, DADiskRef, DADiskGetTypeID);

impl DADisk {
    pub fn from_url(session: &DASession, url: &CFURL) -> Option<DADisk> {
        let ptr = unsafe {
            DADiskCreateFromVolumePath(
                kCFAllocatorDefault,
                session.as_concrete_TypeRef(),
                url.as_concrete_TypeRef(),
            )
        };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe { DADisk::wrap_under_create_rule(ptr) })
        }
    }

    pub fn from_path<P: AsRef<Path>>(session: &DASession, path: P) -> Option<DADisk> {
        let ptr = unsafe {
            DADiskCreateFromBSDName(
                kCFAllocatorDefault,
                session.as_concrete_TypeRef(),
                CString::new(path.as_ref().to_str()?).ok()?.into_raw(),
            )
        };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe { DADisk::wrap_under_create_rule(ptr) })
        }
    }

    pub fn description(&self) -> Option<CFDictionary<CFString>> {
        let ptr = unsafe { DADiskCopyDescription(self.as_concrete_TypeRef()) };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CFDictionary::wrap_under_create_rule(ptr) })
        }
    }
}
