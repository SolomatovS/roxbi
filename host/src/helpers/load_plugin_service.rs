use libloading::{Library, Symbol};
use std::boxed::Box;
use std::error::Error;
use std::ffi::OsString;
use std::os::unix::prelude::OsStrExt;
use std::{collections::HashMap, ffi::OsStr};

pub struct DynamicLibraryManager {
    library: HashMap<OsString, Library>,
}

impl DynamicLibraryManager {
    pub fn add_library(&mut self, path: OsString) -> Result<(), Box<dyn Error>> {
        let lib: Library;
        unsafe {
            lib = Library::new(path.as_os_str())?;
        }

        self.library.insert(path, lib);

        Ok(())
    }

    pub fn find_symbol<T>(&self, symbol: &OsStr) -> Vec<Symbol<T>> {
        self.library
            .iter()
            .filter_map(|x| {
                let sym;
                unsafe {
                    sym = x.1.get(symbol.as_bytes());
                }

                match sym {
                    Ok(sym) => Some(sym),
                    Err(e) => {
                        println!("can't find finction {:?} in {:?}", symbol, x.0);
                        println!("{:?}", e);
                        None
                    }
                }
            })
            .collect()
    }

    pub fn new() -> Self {
        Self { library: HashMap::new() }
    }
}
