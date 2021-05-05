pub mod prelude {
    pub use crate::types::{
        cpu::{CoresLoadInfo, LoadAvg},
        fs::FileSystem,
        mem::{Swap, RAM},
        network::NetworkInterface,
        power::{PowerSources, Type},
        thermal::Fans,
        Error, Result,
    };
    pub use std::time::SystemTime;
}

pub mod imp {
    pub mod network {
        use crate::platform::windows::prelude::*;
        pub fn all() -> Result<Vec<NetworkInterface>> {
            Ok(vec![])
        }
    }

    pub mod cpu {
        use crate::platform::windows::prelude::*;
        pub fn load() -> Result<CoresLoadInfo> {
            Ok(vec![])
        }
        pub fn loadavg() -> Option<LoadAvg> {
            None
        }
    }
    pub mod fs {
        use crate::platform::windows::prelude::*;
        pub fn all() -> Result<Vec<FileSystem>> {
            Ok(vec![])
        }
    }

    pub mod mem {
        use crate::platform::windows::prelude::*;
        pub fn ram() -> Result<RAM> {
            Ok(RAM { used: 0, total: 0 })
        }
        pub fn swap() -> Result<Swap> {
            Ok(Swap {
                used: 0,
                free: 0,
                total: 0,
            })
        }
    }

    pub mod power {
        use crate::platform::windows::prelude::*;
        pub fn sources() -> PowerSources {
            PowerSources {
                sources: None,
                power_type: Type::AC,
                adapter: None,
            }
        }
    }

    pub mod thermal {
        use crate::platform::windows::prelude::*;
        pub fn cpus() -> Option<Vec<f64>> {
            None
        }
        pub fn custom(_key: &str) -> Option<f64> {
            None
        }
        pub fn fans() -> Option<Fans> {
            None
        }
    }

    pub mod boottime {
        use crate::platform::windows::prelude::*;
        pub fn get() -> Option<SystemTime> {
            None
        }
    }
}
