use crate::types::cpu::LoadAvg;
use libc::getloadavg;

pub fn loadavg() -> Option<LoadAvg> {
    let mut res = Default::default();
    if unsafe { getloadavg(&mut res as *mut _ as *mut f64, 3) } == -1 {
        None
    } else {
        Some(res)
    }
}
