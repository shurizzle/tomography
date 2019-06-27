use std::os::raw::c_void;

use core_foundation::base::{kCFAllocatorDefault, CFAllocatorRef, CFTypeID, TCFType};
use core_foundation::{declare_TCFType, impl_TCFType};

#[repr(C)]
pub struct __DASession(c_void);
pub type DASessionRef = *mut __DASession;

#[link(name = "DiskArbitration", kind = "framework")]
extern "C" {
    fn DASessionGetTypeID() -> CFTypeID;
    fn DASessionCreate(allocator: CFAllocatorRef) -> DASessionRef;
}

declare_TCFType! {DASession, DASessionRef}
impl_TCFType!(DASession, DASessionRef, DASessionGetTypeID);

impl DASession {
    pub fn new() -> DASession {
        unsafe { DASession::wrap_under_create_rule(DASessionCreate(kCFAllocatorDefault)) }
    }
}
