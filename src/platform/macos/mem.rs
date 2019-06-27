use crate::types::{mem::*, Error, Result};

use libc::{c_int, c_void, size_t, sysctl, xsw_usage, CTL_HW, CTL_VM, HW_MEMSIZE, VM_SWAPUSAGE};

use super::mach::{
    host_info_t, host_name_port_t, host_statistics, mach_host_self, mach_msg_type_number_t,
    vm_statistics_data_t, MachError, HOST_VM_INFO, HOST_VM_INFO_COUNT,
};

fn get_phys_ram() -> Result<u64> {
    let mib: [c_int; 2] = [CTL_HW, HW_MEMSIZE];
    let mut physical_memory: i64 = 0;
    let mut length: size_t = std::mem::size_of::<i64>() as size_t;

    let err = unsafe {
        sysctl(
            mib.as_ptr() as *mut c_int,
            2,
            &mut physical_memory as *mut _ as *mut c_void,
            &mut length,
            std::ptr::null_mut(),
            0,
        )
    };

    if err != 0 {
        Err(Error::from_errno())
    } else {
        Ok(physical_memory as u64)
    }
}

pub fn ram() -> Result<RAM> {
    let mem_phys = get_phys_ram()?;
    let mut vmstat: vm_statistics_data_t = unsafe { std::mem::zeroed() };
    let mach_host: host_name_port_t = unsafe { mach_host_self() };
    let mut count: mach_msg_type_number_t = HOST_VM_INFO_COUNT;

    let err = unsafe {
        host_statistics(
            mach_host,
            HOST_VM_INFO,
            &mut vmstat as *mut vm_statistics_data_t as host_info_t,
            &mut count,
        )
    };

    if err != 0 {
        Err(Error::new(MachError::new(err)))
    } else {
        let mem_used = vmstat.active_count + vmstat.wire_count;
        let mem_total = mem_used + vmstat.inactive_count + vmstat.free_count;

        Ok(RAM {
            used: (mem_used as f64 / mem_total as f64 * mem_phys as f64) as u64,
            total: mem_phys,
        })
    }
}

pub fn swap() -> Result<Swap> {
    let mib: [c_int; 2] = [CTL_VM, VM_SWAPUSAGE];
    let mut res: xsw_usage = unsafe { std::mem::zeroed() };
    let mut length: size_t = std::mem::size_of::<xsw_usage>() as size_t;

    let err = unsafe {
        sysctl(
            mib.as_ptr() as *mut c_int,
            2,
            &mut res as *mut _ as *mut c_void,
            &mut length,
            std::ptr::null_mut(),
            0,
        )
    };

    if err != 0 {
        Err(Error::from_errno())
    } else {
        Ok(Swap {
            used: res.xsu_used,
            free: res.xsu_avail,
            total: res.xsu_total,
        })
    }
}
