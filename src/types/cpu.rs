use std::fmt;

#[derive(Copy, Clone)]
pub struct CoreLoadInfo {
    pub system: usize,
    pub user: usize,
    pub idle: usize,
}

impl CoreLoadInfo {
    pub fn percent(&self) -> f64 {
        let used = self.system + self.user;
        let total = used + self.idle;
        100f64 * ((used as f64) / (total as f64))
    }
}

impl fmt::Debug for CoreLoadInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CoreLoadInfo")
            .field("system", &self.system)
            .field("user", &self.user)
            .field("idle", &self.idle)
            .field("usage", &format!("{:.2}%", self.percent()))
            .finish()
    }
}

pub type CoresLoadInfo = Vec<CoreLoadInfo>;
