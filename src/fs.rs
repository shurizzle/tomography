use crate::platform::imp::fs;
use std::path::Path;

pub struct FileSystem;

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem
    }

    pub fn all(&self) -> Option<Vec<crate::types::fs::FileSystem>> {
        fs::all().ok()
    }

    pub fn for_device<P: AsRef<Path>>(&self, device: P) -> Option<crate::types::fs::FileSystem> {
        let device = device.as_ref().canonicalize().ok()?;
        self.all()?.into_iter().filter(|fs| fs.device == device).next()
    }

    pub fn for_mountpoint<P: AsRef<Path>>(&self, path: P) -> Option<crate::types::fs::FileSystem> {
        let path = path.as_ref().canonicalize().ok()?;
        self.all()?.into_iter().filter(|fs| fs.mountpoint == path).next()
    }

    pub fn for_label(&self, label: &str) -> Option<crate::types::fs::FileSystem> {
        self.all()?.into_iter().filter(|fs| fs.label == label).next()
    }

    pub fn for_uuid(&self, uuid: &str) -> Option<crate::types::fs::FileSystem> {
        self.all()?.into_iter().filter(|fs| fs.uuid == uuid).next()
    }

    pub fn containing_path<P: AsRef<Path>>(&self, path: P) -> Option<crate::types::fs::FileSystem> {
        let path = path.as_ref().canonicalize().ok()?;
        self.all()?.into_iter().filter(|fs| path.starts_with(&fs.mountpoint)).next()
    }
}
