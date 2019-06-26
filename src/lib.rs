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

cfg_if! {
    if #[cfg(target_os = "macos")] {
        pub mod platform;
        pub use platform::imp::*;
    } else {
        compile_error!("Target OS unsupported");
    }
}

mod cpu;
mod network;

pub use cpu::Cpu;
pub use network::Network;
