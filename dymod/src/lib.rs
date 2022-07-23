
use std::fmt; 
use std::io;
use std::path::Path;
use std::time::SystemTime;

pub use libloading::{Library, Symbol, Error};


#[derive(Debug)]
pub enum DymodError {
  IOError(io::Error),
  LibloadingError(Error),
  DymodNonInitialized,
  SymbolNotFound(Error),
}

impl fmt::Display for DymodError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

#[derive(Debug)]
pub struct DymodSource {
  version: usize,
  modified_time: std::time::SystemTime,
  source_path: String,
  lib_path: String,
  lib: Library,
  manual_reload_needed: bool,
}


fn get_modified_date(file_path: &str) -> Result<SystemTime, DymodError> {
  let metadata = match std::fs::metadata(&file_path) {
    Err(e) => return Err(DymodError::IOError(e)),
    Ok(metadata) => metadata,
  };

  let modified_time = match metadata.modified() {
    Err(e) => return Err(DymodError::IOError(e)),
    Ok(x) => x,
  };

  Ok(modified_time)
}

fn get_new_file_name(file_path: &str, new_version: usize) -> String {
  format!("{}_load_{}", file_path, new_version)
}

impl DymodSource{
  pub fn reload_needed(&self) -> bool {
    // if manual reload turn on
    if self.manual_reload_needed {
      return true;
    }

    // if modified time changed
    match get_modified_date(&&self.source_path) {
      Ok(modified_time) => (modified_time != self.modified_time),
      Err(_) => true,
    }
  }

  pub fn version(&self) -> usize {
    self.version
  }

  pub fn source_file(&self) -> &str {
    &self.source_path
  }

  pub fn dest_file(&self) -> &str {
    &self.lib_path
  }

  pub fn create_new_version(&self) -> Result<DymodSource, DymodError> {
    let new_lib = DymodSource::new(&self.source_path,  self.version+1)?;

    Ok(new_lib)
  }

  pub fn get_lib_ref(&self) -> Result<&Library, DymodError> {
    return Ok(&self.lib)
  }

  pub fn reload_turn_on(&mut self) {
    self.manual_reload_needed = true
  }

  pub fn new(file_path: &str, version: usize) -> Result<Self, DymodError> {
    let mut new_version: usize = version;

    let copy_to = loop {
      let path = get_new_file_name(file_path, new_version);

      if !Path::new(&path).exists() {
        break path;
      }

      new_version = new_version + 1;
    };
    
    if let Err(e) = std::fs::copy(&file_path, &copy_to) {
      return Err(DymodError::IOError(e))
    }

    match unsafe {Library::new(&copy_to)} {
      Ok(lib) => {
        Ok(DymodSource {
          modified_time: get_modified_date(&file_path)?,
          version: new_version,
          source_path: String::from(file_path),
          lib_path: String::from(copy_to),
          lib,
          manual_reload_needed: false,
        })
      },
      Err(e) => Err(DymodError::LibloadingError(e)),
    }
  }
}


#[macro_export]
macro_rules! dymod {
    (
      #[struct = $struct_name: ident]
      pub mod $modname: ident {
        $(fn $fnname: ident ( $($argname: ident : $argtype: ty),* $(,)? ) $(-> $returntype: ty)? ;)*
      }
    ) => {
        pub mod $modname {
            use super::*;

            use $crate::{Library, Symbol};
            use $crate::{DymodError, DymodSource};

            use std::sync::RwLock;

            pub struct $struct_name {
              dy: RwLock<DymodSource>
            }

            impl $struct_name {
              $(
                pub fn $fnname(&self, $($argname: $argtype),*) -> Result<$($returntype)?, DymodError> {
                  let bor = loop {
                    let bor = self.dy.read().unwrap();
  
                    if !bor.reload_needed() {
                      break bor;
                    }
                    
                    let mut dy = self.dy.write().unwrap();
                    *dy = dy.create_new_version()?;
                  };

                  let lib = bor.get_lib_ref()?;
                    let symbol = unsafe {
                      lib.get(stringify!($fnname).as_bytes())
                    };

                    let symbol: Symbol<extern fn($($argtype),*) $(-> $returntype)?> = match symbol {
                      Ok(sym) => sym,
                      Err(e) => return Err(DymodError::SymbolNotFound(e)),
                    };
                    
                    Ok(symbol($($argname),*))
                }
              )*
            }

            pub fn load(file_path: &str) -> Result<$struct_name, DymodError> {
              let dy = match DymodSource::new(&file_path, 1) {
                Ok(dy) => dy,
                Err(e) => return Err(e),
              };

              let sdf = $struct_name {
                dy: RwLock::new(dy),
              };

              Ok(sdf)
            }
        }
    }
}
