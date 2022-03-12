use std::collections::HashMap;
use std::ffi::OsString;
use std::{ffi::OsStr};
use std::error::Error;
//use std::fmt::{Display, Formatter};
use std::boxed::Box;
use libloading::{Library, Symbol};
use std::rc::Rc;

pub struct ExtensionsLoader {
    library: HashMap<OsString, Library>
}

impl ExtensionsLoader {
    pub fn add_library(&mut self, path: OsString) -> Result<(), Box<dyn Error>>
    {
        let lib: Library;
        unsafe {
            lib = Library::new(path.as_os_str())?;
        }
        
        self.library.insert(path, lib);

        Ok(())
    }

    pub fn new() -> Self {
        Self {
            library: HashMap::new()
        }
    }
    /*
    pub fn get_symbol<'a, T : 'a>(&self, path: &OsStr, symbol: &[u8]) -> Result<Symbol<T>, Box<dyn Error>>
    {
        unsafe {
            let lib = self.library
            let sym: Symbol<T> = lib.get(symbol)?;

            Ok(sym)
        }
    }
    */
}
