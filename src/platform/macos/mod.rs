extern crate core_foundation;
extern crate ifstructs;
extern crate ioctl_rs;
extern crate smc;

pub mod cpu;
pub mod fs;
pub mod mach;
pub mod mem;
pub mod netif;
pub mod power;
pub mod thermal;
pub mod wifi;

pub mod imp {
    pub mod network {
        pub use super::super::netif::all;
    }

    pub mod cpu {
        pub use super::super::cpu::load;
    }

    pub mod fs {
        pub use super::super::fs::all;
    }

    pub mod mem {
        pub use super::super::mem::ram;
    }

    pub mod power {
        pub use super::super::power::all;
    }

    pub mod thermal {
        pub use super::super::thermal::{cpus, fans};
    }
}
