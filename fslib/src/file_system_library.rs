use std::ffi::{OsStr, OsString};
use std::fmt::{Display};
use std::boxed::Box;
use std::os::unix::ffi::OsStrExt;

use lib::ILibrary;
use libloading::{Library, Symbol};

type Error = Box<dyn std::error::Error>;

pub struct FileSystemLibrary {
    path: OsString,
    lib: Library,
    //symbols: HashMap<OsString, Symbol>
    source_identifier: OsString,
}

impl FileSystemLibrary {
    pub fn new(path: OsString, source_identifier: OsString) -> Result<Self, Error> {
        let lib = Self::build(&path)?;

        Ok(Self {
            lib,
            path,
            source_identifier,
        })
    }

    fn build(path: &OsString) -> Result<Library, Error> {
        unsafe { Ok(Library::new(path)?) }
    }

    pub fn find<T>(&self, symbol: &OsStr) -> Result<Symbol<T>, Error> {
        unsafe {
            return Ok(self.lib.get(symbol.as_bytes())?);
        }
    }
}

impl ILibrary for FileSystemLibrary {
    fn id(&self) -> &OsStr {
        &self.source_identifier
    }
}

impl Display for FileSystemLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p} ({:?})", &self, self.path)
    }
}
