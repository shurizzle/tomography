use crate::types::thermal::*;
use std::collections::HashMap;

use smc::{cpu_temp, fans_rpm};

pub fn fans() -> Option<Fans> {
    let fans = fans_rpm()?;
    let mut hm = HashMap::with_capacity(fans.len());
    for fan in fans.into_iter() {
        hm.insert(
            fan.name,
            Fan {
                min_speed: fan.min_speed,
                actual_speed: fan.actual_speed,
                max_speed: fan.max_speed,
            },
        );
    }

    Some(hm)
}

pub fn cpus() -> Vec<f64> {
    vec![cpu_temp()]
}
