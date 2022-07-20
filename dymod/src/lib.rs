#![allow(clippy::needless_doctest_main)]

#[cfg(all(target_arch = "wasm32", feature = "force-dynamic"))]
compile_error!("The force-dynamic feature is not supported on WASM targets.");

#[cfg(any(
    feature = "force-dynamic",
    all(not(feature = "force-static"), debug_assertions)
))]
#[doc(hidden)]
pub use libloading::{Library, Symbol, Error};

#[cfg(any(
    feature = "force-dynamic",
    all(not(feature = "force-static"), debug_assertions)
))]
#[doc(hidden)]
pub const AUTO_RELOAD: bool = cfg!(feature = "auto-reload");

#[cfg(any(
    feature = "force-static",
    all(not(feature = "force-dynamic"), not(debug_assertions))
))]
#[macro_export]
macro_rules! dymod {
    (
        #[path = $libpath: tt]
        pub mod $modname: ident {
            $(fn $fnname: ident ( $($argname: ident : $argtype: ty),* $(,)? ) $(-> $returntype: ty)? ;)*
        }
    ) => {
        #[path = $libpath]
        pub mod $modname;
    }
}

#[cfg(any(
    feature = "force-dynamic",
    all(not(feature = "force-static"), debug_assertions)
))]
#[macro_export]
macro_rules! dymod {
    (
      #[path = $libpath: tt]
      pub mod $modname: ident {
        $(fn $fnname: ident ( $($argname: ident : $argtype: ty),* $(,)? ) $(-> $returntype: ty)? ;)*
      }
    ) => {
        pub mod $modname {
            use super::*;
            use std::fs;
            use std::result::Result;
            use std::error::Error;
            use std::path::Path;

            use std::{thread, time, io};

            use $crate::{Library, Symbol};

            #[derive(Debug)]
            pub enum DymodError {
                LibloadingNotFound($crate::Error),
                NoneError,
                SymbolNotFound($crate::Error),
                LibraryCopyError(io::Error),
                RemoveOldLibraryError(io::Error),
            }

            static mut VERSION: usize = 0;

            static mut DYLIB: Option<Library> = None;
            static mut MODIFIED_TIME: Option<std::time::SystemTime> = None;

            const DYLIB_PATH: &'static str = stringify!($libpath);
            
            pub fn reload() -> Result<Library, DymodError> {
              let path = unsafe {
                DYLIB = None;

                let new_path = format!("{}{}", DYLIB_PATH.trim_matches('"'), VERSION);
                let new_path_ = Path::new(&new_path);

                if new_path_.exists() {
                  if let Err(e) = fs::remove_file(new_path_) {
                    return Err(DymodError::RemoveOldLibraryError(e))
                  }
                }
                
                println!("{} -> {}", DYLIB_PATH.trim_matches('"'), &new_path);
                if let Err(e) = std::fs::copy(DYLIB_PATH.trim_matches('"'), &new_path) {
                  println!("{}", e);
                  return Err(DymodError::LibraryCopyError(e))
                }
                
                new_path
              };
              
              unsafe {
                VERSION += 1;

                match Library::new(&path) {
                  Ok(lib) => Ok(lib),
                  Err(e) => Err(DymodError::LibloadingNotFound(e)),
                }
              }
            }

            fn dymod_file_changed() -> bool {
                fn file_changed() -> Result<bool, std::io::Error> {
                    let metadata = std::fs::metadata(&DYLIB_PATH)?;
                    let modified_time = metadata.modified()?;
                    unsafe {
                        let changed = MODIFIED_TIME.is_some() && MODIFIED_TIME != Some(modified_time);
                        MODIFIED_TIME = Some(modified_time);
                        Ok(changed)
                    }
                }

                $crate::AUTO_RELOAD && file_changed().unwrap_or(false)
            }

            fn get_lib() -> Result<&'static Library, DymodError> {
              unsafe {
                if DYLIB.is_none() || (DYLIB.is_some() && dymod_file_changed()) {
                  DYLIB = Some(reload()?);
                }

                match &DYLIB {
                  None => return Err(DymodError::NoneError),
                  Some(lib) => Ok(lib),
                }
              }
            }

            $(
            pub fn $fnname($($argname: $argtype),*) -> Result<$($returntype)?, DymodError> {
              let lib = get_lib()?;
              unsafe {
                let symbol: Symbol<extern fn($($argtype),*) $(-> $returntype)?> = match lib.get(stringify!($fnname).as_bytes()) {
                  Ok(sym) => sym,
                  Err(e) => return Err(DymodError::SymbolNotFound(e)),
                };
                
                Ok(symbol($($argname),*))
              }
            }
            )*
        }
    }
}
