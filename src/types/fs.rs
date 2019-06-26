#[derive(Debug)]
pub struct FileSystem {
    pub name: String,
    pub filesystem: String,
    pub mountpoint: String,
    pub total: u64,
    pub free: u64,
    pub used: u64,
}
