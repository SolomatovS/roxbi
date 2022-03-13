#![allow(dead_code)]

use std::error::Error;
use std::ffi::OsString;
use libloading::Library;


use ilibrary::ILibrary;

struct  FileSystemLibrary{
    path: OsString,
    lib: Option<Library>,
}

impl FileSystemLibrary {
    fn new(path: OsString) -> Self {
        Self{
            path,
            lib: None
        }
    }
}

impl ILibrary for FileSystemLibrary {
    fn build(&mut self) -> Result<(), Box<dyn Error>> {
        unsafe {
            self.lib = Some(Library::new(self.path.as_os_str())?);
        }

        Ok(())
    }

    fn get(&self) -> Option<&Library> {
        match &(self.lib) {
            None => None,
            Some(k) => Some(&k)
        }
    }
}
