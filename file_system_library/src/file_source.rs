use std::ffi::OsString;

use lib::ILibrary;

use super::file_system_library::FileSystemLibrary;

pub struct FileSource {
    path: OsString,
}

impl FileSource {
    pub fn new(path: OsString) -> Self {
        Self { path }
    }

    pub fn build_library(&self) -> Box<dyn ILibrary> {
        Box::new(FileSystemLibrary::new(self.path.clone(), self.path.clone()))
    }
}
