use std::ffi::{OsStr, OsString};
use std::fmt::{Display};
use std::boxed::Box;
use std::os::unix::ffi::OsStrExt;

use lib::ILibrary;
use libloading::{Library, Symbol};
use std::any::Any;

type Error = Box<dyn std::error::Error>;

pub struct FileSystemLibrary {
    path: OsString,
    lib: Library,
    //symbols: HashMap<OsString, Symbol>
    id: OsString,
}

impl FileSystemLibrary {
    pub fn new(path: OsString, id: OsString) -> Result<Self, Error> {
        let lib = Self::build(&path)?;

        Ok(Self {
            lib,
            path,
            id,
        })
    }

    fn build(path: &OsString) -> Result<Library, Error> {
        unsafe { Ok(Library::new(path)?) }
    }

    /*
    pub fn find<T>(&self, path: &OsStr) -> Result<Symbol<T>, Error> {
        let symbol: Symbol<T>;
        let path = path.as_bytes();

        unsafe {
            symbol = self.lib.get(path)?
        }

        Ok(symbol)
    }
*/
}
/*
impl ILibrary for FileSystemLibrary {
    fn id(&self) -> &OsStr {
        &self.id
    }

    fn find<'a, T: 'a>(&'a self, path: &OsStr) -> Result<Box<dyn Any + 'a>, Error>
    {
        let symbol: Symbol<T>;
        let path = path.as_bytes();
        unsafe {
            symbol = self.lib.get(path)?
        }

        let symbol = Box::new(symbol);

        Ok(symbol)
    }
}
*/

impl Display for FileSystemLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p} ({:?})", &self, self.path)
    }
}
