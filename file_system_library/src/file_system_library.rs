#![allow(dead_code)]

use std::{ffi::{OsString, OsStr}, fmt::Display};
use libloading::Library;
use ilibrary::ILibrary;


type Error = Box<dyn std::error::Error>;

pub struct  FileSystemLibrary {
    path: OsString,
    lib: Option<Library>,
    source_identifier: OsString,
}

impl FileSystemLibrary {
    pub fn new(path: OsString, source_identifier: OsString) -> Self {
        Self {
            lib: None,
            path,
            source_identifier,
        }
    }

    fn build(path: &OsString) -> Result<Library, Error> {
        unsafe {
            Ok(Library::new(path)?)
        }
    }
}

impl ILibrary for FileSystemLibrary {
    fn reload(&mut self) -> Result<(), Error> {
        self.lib = Some(Self::build(&self.path)?);

        Ok(())
    }

    fn check(&self) -> Result<(), Error> {
        Self::build(&self.path)?;

        Ok(())
    }

    fn identifier(&self) -> &OsStr {
        &self.source_identifier
    }
}

impl Display for FileSystemLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p} ({:?})", &self, self.path)
    }
}

