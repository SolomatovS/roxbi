#![allow(clippy::needless_doctest_main)]

#[cfg(all(target_arch = "wasm32", feature = "force-dynamic"))]
compile_error!("The force-dynamic feature is not supported on WASM targets.");

#[cfg(any(
    feature = "force-dynamic",
    all(not(feature = "force-static"), debug_assertions)
))]
#[doc(hidden)]
pub use libloading::{Library, Symbol};

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

            use $crate::{Library, Symbol};

            static mut VERSION: usize = 0;

            static mut DYLIB: Option<Library> = None;
            static mut MODIFIED_TIME: Option<std::time::SystemTime> = None;

            static mut DYLIB_PATH: Option<&str> = None;
            // const DYLIB_PATH: &'static str = path;
            // static DYLIB_PATH: &'str = "/Users/solomatovs/Documents/GitHub/roxbi/target/debug/libsubcrate.dylib";

            pub fn set_lib(lib_path: &'static str) {
                DYLIB_PATH = Some(lib_path);
            }

            pub enum DymodError {
                IOError(std::io::Error)
            }

            impl From<std::io::Error> for DymodError {
                fn from(err: std::io::Error) -> DymodError {
                    DymodError::Parse(err)
                }
            }

            pub fn reload() -> Result<(), DymodError>  {
                //let path = unsafe {
                    match DYLIB_PATH {
                        Some(lib_path) => {
                            // Clean up the old
                            if DYLIB.is_some() {
                                let old_path = format!("{}{}", lib_path, VERSION - 1);
                                match std::fs::remove_file(&old_path) {
                                    Err(e) => return Err(DymodError::IOError(e)),
                                }
                            }

                            // Create the new
                            let new_path = format!("{}{}", lib_path, VERSION);
                            std::fs::copy(lib, &new_path).expect("Failed to copy new dylib");

                            // Load new version
                            unsafe {
                                let lib = Library::new(&new_path);
                                
                                DYLIB = Some(lib?)
                            }

                            VERSION += 1;
                        }
                    }
                //};
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

            fn dymod_get_lib() -> &'static Library {
                unsafe {
                    if DYLIB.is_none() || dymod_file_changed() {
                        reload();
                    }
                    DYLIB.as_ref().unwrap()
                }
            }

            $(
            pub fn $fnname($($argname: $argtype),*) $(-> $returntype)? {
                let lib = dymod_get_lib();
                let stringify_func = stringify!($fnname);
                let bytes_name = stringify_func.as_bytes();

                let symbol: Symbol<extern fn($($argtype),*) $(-> $returntype)?> = unsafe {
                    lib.get(bytes_name).expect("Failed to get symbol from dylib")
                };

                symbol($($argname),*)
            }
            )*
        }
    }
}
