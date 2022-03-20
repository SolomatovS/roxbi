#![allow(dead_code)]

use std::{ffi::OsString, fmt::Display};
use libloading::Library;
use ilibrary::ILibrary;


type Error = Box<dyn std::error::Error>;

pub struct  FileSystemLibrary{
    path: OsString,
    lib: Library,
}

impl FileSystemLibrary {
    pub fn new(path: OsString) -> Result<Self, Error> {
        Ok(Self {
            lib: Self::build(&path)?,
            path,
        })
    }

    fn build(path: &OsString) -> Result<Library, Error> {
        unsafe {
            Ok(Library::new(path)?)
        }
    }
}

impl ILibrary for FileSystemLibrary {

}

impl Display for FileSystemLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p} ({:?})", &self, self.path)
    }
}