use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileSystem {
    pub device: PathBuf,
    pub filesystem: String,
    pub mountpoint: PathBuf,
    pub label: String,
    pub uuid: String,
    pub total: u64,
    pub free: u64,
    pub used: u64,
}
