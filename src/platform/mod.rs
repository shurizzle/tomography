#[cfg(unix)]
mod unix;

cfg_if! {
    if #[cfg(target_os = "macos")] {
        mod macos;
        pub use macos::imp;
    } else if #[cfg(windows)] {
        mod windows;
        pub use windows::imp;
    } else {
        compile_error!("Target OS unsupported");
    }
}
