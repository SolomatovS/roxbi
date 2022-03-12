use std::collections::binary_heap::Iter;
use std::os::unix::prelude::OsStrExt;
use std::{collections::HashMap, ffi::OsStr};
use std::ffi::OsString;
use std::error::Error;
use std::boxed::Box;
use libloading::{Library, Symbol};


pub trait IDLibRepository {
    fn find_symbol<T>(&self, symbol: &OsStr) -> Vec<Symbol<T>>;
    fn add(self, source: Box<dyn IDLibRepositorySource>) -> Self;
}

pub struct DLibRepository {
    source: Vec<Box<dyn IDLibRepositorySource>>
}

impl DLibRepository {
    pub fn new() -> Self {
        Self {
            source: Vec::new()
        }
    }
}

impl IDLibRepository for DLibRepository {
    fn add(mut self, s: Box<dyn IDLibRepositorySource>) -> Self{
        self.source.push(s);

        self
    }

    fn find_symbol<T>(&self, symbol: &OsStr) -> Vec<Symbol<T>>{
        self.source
            .iter()
            .flat_map(|s| s.get_libs())
            .map(|x| x.get())
            .filter_map(|x| {
                let sym;
                unsafe {
                    sym = x.get(symbol.as_bytes());
                }

                match sym {
                    Ok(s) => Some(s),
                    Err(e) => {
                        println!("can't find finction {:?} in {:?}", symbol, x);
                        println!("{:?}", e);
                        None
                    }
                }
            })
            .collect()
    }
}

trait ILibrary {
    fn get(&self) -> &Library;
}

struct  FileSystemLibrary{
    path: OsString,
    lib: Library,
}

impl FileSystemLibrary {
    fn new(path: OsString, lib: Library) -> Self {
        Self{
            path,
            lib,
        }
    }
}

impl ILibrary for FileSystemLibrary {
    fn get(&self) -> &Library {
        &self.lib
    }
}

pub trait IDLibRepositorySource {
    fn get_libs(&self) -> &Vec<Box<dyn ILibrary>>;
}

pub struct  FileSystemDinamicLibrarySource {
    library: Vec<FileSystemLibrary>
}

impl FileSystemDinamicLibrarySource {
    pub fn new() -> Self {
        Self {
            library: Vec::new()
        }
    }

    pub fn add_from_path(&mut self, path: OsString) -> Result<(), Box<dyn Error>>
    {
        let lib: Library;
        unsafe {
            lib = Library::new(path.as_os_str())?;
        }
        
        self.library.push(FileSystemLibrary::new(path, lib));

        Ok(())
    }
}

impl IDLibRepositorySource for FileSystemDinamicLibrarySource {
    fn get_libs(&self) -> &Vec<Box<dyn ILibrary>> {
        
        let sdf = self.library.iter()
            .map(|x| Box::new(x))
            .collect()
        ;

    }
}


/*
impl IDLibRepositorySource for FileSystemDinamicLibrarySource {
    fn build() -> Box<dyn IDLibRepository> {
        DLibRepository {

        }
    }
}
*/
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

    pub fn find_symbol<T>(&self, symbol: &OsStr) -> Vec<Symbol<T>>
    {
        self.library.iter()
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
        Self {
            library: HashMap::new()
        }
    }
}