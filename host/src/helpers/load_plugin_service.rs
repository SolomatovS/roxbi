use std::collections::HashMap;
use std::ffi::OsString;
use std::{ffi::OsStr};
use std::error::Error;
//use std::fmt::{Display, Formatter};
use std::boxed::Box;
use libloading::{Library, Symbol};
use std::rc::Rc;

pub struct DynamicLibraryManager {
    library: HashMap<OsString, Library>
}

impl DynamicLibraryManager {
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

    pub fn get_symbol<'a, T : 'a>(&self, symbol: &[u8]) -> Vec<Symbol<T>>
    {
        self.library.iter()
            .filter_map(|x| {
                unsafe {
                    match x.1.get(symbol) {
                        Ok(sym) => Some(sym),
                        Err(_) => None
                    }
                }
            })
            .collect()
    }
}
