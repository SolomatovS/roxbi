use std::collections::HashMap;
use std::ffi::OsString;
use std::error::Error;
use std::boxed::Box;
use libloading::{Library, Symbol};


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

    pub fn find_symbol<T>(&self, symbol: &[u8]) -> Vec<Symbol<T>>
    {
        self.library.iter()
            .filter_map(|x| {
                unsafe {
                    match x.1.get(symbol) {
                        Ok(sym) => Some(sym),
                        Err(e) => {
                            println!("{:?}", e);
                            None
                        }
                    }
                }
            })
            .collect()
    }

    pub fn new() -> Self {
        Self {
            library: HashMap::new()
        }
    }
}