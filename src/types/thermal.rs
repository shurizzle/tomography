use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Fan {
    pub min_speed: f64,
    pub current_speed: f64,
    pub max_speed: f64,
}

impl Fan {
    pub fn rpm(&self) -> f64 {
        let mut rpm = self.current_speed - self.min_speed;
        if rpm < 0.0 {
            rpm = 0.0;
        }

        rpm
    }

    pub fn percent(&self) -> f64 {
        self.rpm() / (self.max_speed - self.min_speed) * 100.0
    }
}

impl fmt::Debug for Fan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Fan")
            .field("min_speed", &self.min_speed)
            .field("current_speed", &self.current_speed)
            .field("max_speed", &self.max_speed)
            .field("rpm", &self.rpm())
            .field("percent", &self.percent())
            .finish()
    }
}

pub type Fans = HashMap<String, Fan>;
