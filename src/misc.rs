use crate::platform::imp::boottime;
use std::time::SystemTime;

pub struct Misc;

impl Misc {
    pub fn new() -> Misc {
        Misc
    }

    pub fn boot_time(&self) -> Option<SystemTime> {
        boottime::get()
    }
}
