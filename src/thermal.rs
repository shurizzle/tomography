use crate::platform::imp::thermal;
use crate::types::thermal::*;

pub struct Thermal;

impl Thermal {
    pub fn new() -> Thermal {
        Thermal
    }

    pub fn fans(&self) -> Option<Fans> {
        thermal::fans()
    }

    pub fn cpus(&self) -> Option<Vec<f64>> {
        thermal::cpus()
    }

    pub fn custom(&self, key: &str) -> Option<f64> {
        thermal::custom(key)
    }
}
