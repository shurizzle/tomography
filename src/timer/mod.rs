pub mod perfected_stated_timer;
pub mod stated_timer;
pub mod timer;

pub use perfected_stated_timer::PerfectedStatedTimer as Timer;
pub use stated_timer::StatedTimer;
pub use timer::Timer as SimpleTimer;
