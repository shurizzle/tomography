#[macro_use]
extern crate cfg_if;
#[cfg(unix)]
extern crate errno;
extern crate libc;
extern crate rug;

pub mod perfecter;
pub mod timer;
pub mod types;

pub use perfecter::Perfecter;
pub use timer::Timer;

pub mod platform;
pub use platform::imp::*;

mod cpu;
mod fs;
mod mem;
mod misc;
mod network;
mod power;
mod thermal;

pub use cpu::Cpu;
pub use fs::FileSystem;
pub use mem::Memory;
pub use misc::Misc;
pub use network::Network;
pub use power::Power;
pub use thermal::Thermal;
