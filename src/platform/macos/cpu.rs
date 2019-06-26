use crate::types::cpu::{CoreLoadInfo, CoresLoadInfo};
use crate::types::Error;

use super::mach::{
    host_processor_info, mach_host_self, mach_msg_type_number_t, natural_t,
    processor_cpu_load_info, processor_cpu_load_info_t, processor_info_array_t, vm_address_t,
    vm_deallocate, vm_size_t, MachError, CPU_STATE_IDLE, CPU_STATE_NICE, CPU_STATE_SYSTEM,
    CPU_STATE_USER, PROCESSOR_CPU_LOAD_INFO,
};

pub fn load() -> Result<CoresLoadInfo, Error> {
    let mut cpu_load: processor_cpu_load_info_t = unsafe { std::mem::zeroed() };
    let mut _processor_msg_count: mach_msg_type_number_t = unsafe { std::mem::zeroed() };
    let mut processor_count: natural_t = 0;

    let mach_host = unsafe { mach_host_self() };

    let err = unsafe {
        host_processor_info(
            mach_host,
            PROCESSOR_CPU_LOAD_INFO,
            &mut processor_count,
            (&mut cpu_load as *mut processor_cpu_load_info_t) as *mut processor_info_array_t,
            &mut _processor_msg_count,
        )
    };

    if err != 0 {
        Err(Error::new(MachError::new(err)))
    } else {
        let mut cpus: CoresLoadInfo = Vec::with_capacity(processor_count as usize);

        for i in 0..processor_count {
            let cpu = unsafe {
                let base_addr = cpu_load as vm_address_t;
                let struct_size = std::mem::size_of::<processor_cpu_load_info>() as vm_address_t;
                let ii = i as vm_address_t;
                let struct_pointer = (base_addr + struct_size * ii) as processor_cpu_load_info_t;
                *struct_pointer
            };

            let cpu = CoreLoadInfo {
                system: cpu.cpu_ticks[CPU_STATE_SYSTEM] as usize,
                user: (cpu.cpu_ticks[CPU_STATE_USER] + cpu.cpu_ticks[CPU_STATE_NICE]) as usize,
                idle: cpu.cpu_ticks[CPU_STATE_IDLE] as usize,
            };

            cpus.push(cpu);
        }

        unsafe {
            vm_deallocate(
                mach_host,
                cpu_load as vm_address_t,
                (std::mem::size_of::<processor_cpu_load_info>() as vm_size_t)
                    * (processor_count as vm_size_t),
            )
        };

        Ok(cpus)
    }
}
