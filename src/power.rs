use crate::platform::imp::power;
use crate::types::power::PowerSources;

pub struct Power;

impl Power {
    pub fn new() -> Power {
        Power
    }

    pub fn sources(&self) -> PowerSources {
        power::sources()
    }
}
