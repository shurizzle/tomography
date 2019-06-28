use std::fmt;

#[derive(Copy, Clone)]
pub struct RAM {
    pub used: u64,
    pub total: u64,
}

#[derive(Copy, Clone)]
pub struct Swap {
    pub used: u64,
    pub free: u64,
    pub total: u64,
}

impl RAM {
    pub fn percent(&self) -> f64 {
        f64::from(100) * ((self.used as f64) / (self.total as f64))
    }
}

impl Swap {
    pub fn percent(&self) -> f64 {
        if self.total == 0 {
            0f64
        } else {
            f64::from(100) * ((self.used as f64) / (self.total as f64))
        }
    }
}

impl fmt::Debug for RAM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RAM")
            .field("used", &self.used)
            .field("total", &self.total)
            .field("percent", &self.percent())
            .finish()
    }
}

impl fmt::Debug for Swap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Swap")
            .field("used", &self.used)
            .field("free", &self.free)
            .field("total", &self.total)
            .field("percent", &self.percent())
            .finish()
    }
}
