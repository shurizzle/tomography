cfg_if! {
    if #[cfg(target_os = "macos")] {
        mod macos;
        pub use macos::imp;
    } else {
        compile_error!("Target OS unsupported");
    }
}
