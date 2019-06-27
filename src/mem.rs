use crate::platform::imp::mem;
use crate::types::mem::{Swap, RAM};

pub struct Memory;

impl Memory {
    pub fn new() -> Memory {
        Memory
    }

    pub fn ram(&self) -> Option<RAM> {
        mem::ram().ok()
    }

    pub fn swap(&self) -> Option<Swap> {
        mem::swap().ok()
    }
}
