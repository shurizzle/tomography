pub mod cpu;
pub mod error;
pub mod fs;
pub mod mem;
pub mod network;
pub mod power;
pub mod thermal;

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
