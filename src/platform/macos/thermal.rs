use crate::types::thermal::*;
use std::collections::HashMap;

use smc::SMC;

pub fn fans() -> Option<Fans> {
    let smc = SMC::shared().ok()?;
    let fans = smc.fans().ok()?;
    let mut hm = HashMap::with_capacity(fans.len());

    for fan in fans.into_iter() {
        hm.insert(
            fan.name().to_string(),
            Fan {
                min_speed: fan.min_speed().ok()?,
                current_speed: fan.current_speed().ok()?,
                max_speed: fan.max_speed().ok()?,
            },
        );
    }

    Some(hm)
}

pub fn cpus() -> Option<Vec<f64>> {
    let smc = SMC::shared().ok()?;
    smc.cpus_temperature().ok()
}

pub fn custom(key: &str) -> Option<f64> {
    let smc = SMC::shared().ok()?;
    smc.temperature(key.into()).ok()
}
