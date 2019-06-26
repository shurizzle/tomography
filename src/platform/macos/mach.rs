#![allow(non_camel_case_types)]

use libc::strlen;
use std::os::raw::{c_int, c_uint, c_ulong};
use std::{fmt, slice, str};

pub type __darwin_natural_t = c_uint;
pub type natural_t = __darwin_natural_t;
pub type mach_port_name_t = natural_t;
pub type mach_port_t = mach_port_name_t;
pub type host_t = mach_port_t;
pub type vm_map_t = mach_port_t;
pub type host_name_port_t = host_t;
pub type uintptr_t = c_ulong;
pub type vm_size_t = uintptr_t;
pub type vm_offset_t = uintptr_t;
pub type vm_address_t = vm_offset_t;
pub type processor_cpu_load_info_t = *mut processor_cpu_load_info;
pub type vm_statistics_data_t = vm_statistics;
// pub type vm_statistics_t = *mut vm_statistics;
pub type mach_msg_type_number_t = natural_t;
pub type kern_return_t = c_int;
pub type integer_t = c_int;
pub type processor_info_array_t = *mut integer_t;
pub type host_info_t = *mut integer_t;
pub type processor_flavor_t = integer_t;
pub type host_flavor_t = integer_t;

#[repr(C)]
#[derive(Copy, Debug)]
pub struct processor_cpu_load_info {
    pub cpu_ticks: [natural_t; CPU_STATE_MAX],
}

impl std::clone::Clone for processor_cpu_load_info {
    fn clone(&self) -> Self {
        *self
    }
}

impl std::default::Default for processor_cpu_load_info {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Debug)]
pub struct vm_statistics {
    pub free_count: natural_t,
    pub active_count: natural_t,
    pub inactive_count: natural_t,
    pub wire_count: natural_t,
    pub zero_fill_count: natural_t,
    pub reactivations: natural_t,
    pub pageins: natural_t,
    pub pageouts: natural_t,
    pub faults: natural_t,
    pub cow_faults: natural_t,
    pub lookups: natural_t,
    pub hits: natural_t,
    pub purgeable_count: natural_t,
    pub purges: natural_t,
    pub speculative_count: natural_t,
}

impl std::clone::Clone for vm_statistics {
    fn clone(&self) -> Self {
        *self
    }
}

impl std::default::Default for vm_statistics {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

pub const CPU_STATE_USER: usize = 0usize;
pub const CPU_STATE_SYSTEM: usize = 1usize;
pub const CPU_STATE_IDLE: usize = 2usize;
pub const CPU_STATE_NICE: usize = 3usize;
pub const CPU_STATE_MAX: usize = 4usize;

pub const PROCESSOR_CPU_LOAD_INFO: processor_flavor_t = 2;

// pub const HOST_LOAD_INFO: host_flavor_t = 1;
pub const HOST_VM_INFO: host_flavor_t = 2;
// pub const HOST_CPU_LOAD_INFO: host_flavor_t = 3;
pub const HOST_VM_INFO_COUNT: mach_msg_type_number_t = (std::mem::size_of::<vm_statistics>()
    / std::mem::size_of::<integer_t>())
    as mach_msg_type_number_t;

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    pub fn mach_host_self() -> host_name_port_t;
    pub fn host_processor_info(
        host: host_t,
        flavor: processor_flavor_t,
        out_processor_count: *mut natural_t,
        out_processor_info: *mut processor_info_array_t,
        out_processor_infoCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn host_statistics(
        host: host_t,
        flavor: host_flavor_t,
        host_info_out: host_info_t,
        host_info_outCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn vm_deallocate(host: vm_map_t, address: vm_address_t, size: vm_size_t) -> kern_return_t;
    fn mach_error_string(code: kern_return_t) -> *const u8;
}

fn get_error_string(code: kern_return_t) -> &'static str {
    let ptr = unsafe { mach_error_string(code) };

    if ptr.is_null() {
        "Unknown error"
    } else {
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr, strlen(ptr as *const i8))) }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MachError(kern_return_t);

impl MachError {
    pub fn new(code: kern_return_t) -> MachError {
        MachError(code)
    }

    #[inline]
    pub fn code(&self) -> c_int {
        self.0
    }
}

impl std::error::Error for MachError {
    fn description(&self) -> &str {
        get_error_string(self.code())
    }
}

impl fmt::Display for MachError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", get_error_string(self.code()))
    }
}

impl fmt::Debug for MachError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MachError")
            .field("code", &self.0)
            .field("description", &get_error_string(self.code()))
            .finish()
    }
}
