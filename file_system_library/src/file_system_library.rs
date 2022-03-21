#![allow(dead_code)]

use std::{ffi::OsString, fmt::Display};
use libloading::Library;
use ilibrary::ILibrary;


type Error = Box<dyn std::error::Error>;

pub struct  FileSystemLibrary {
    id: u32,
    path: OsString,
    lib: Option<Library>,
}

impl FileSystemLibrary {
    pub fn new(id:u32, path: OsString) -> Self {
        Self {
            id,
            lib: None, //Self::build(&path)?,
            path,
        }
    }

    fn build(path: &OsString) -> Result<Library, Error> {
        unsafe {
            Ok(Library::new(path)?)
        }
    }
}

impl ILibrary for FileSystemLibrary {
    fn id(&self) -> u32 {
        self.id
    }

    fn reload(&mut self) -> Result<(), Error> {
        self.lib = Some(Self::build(&self.path)?);

        Ok(())
    }

    fn check(&self) -> Result<(), Error> {
        Self::build(&self.path)?;

        Ok(())
    }
}

impl Display for FileSystemLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p} ({:?})", &self, self.path)
    }
}