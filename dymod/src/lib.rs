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

            #[cfg(target_os = "macos")]
            const DYLIB_PATH: &'static str = concat!(stringify!($libpath), ".dylib");
            #[cfg(all(unix, not(target_os = "macos")))]
            const DYLIB_PATH: &'static str = concat!(stringify!($libpath), ".so");
            #[cfg(windows)]
            const DYLIB_PATH: &'static str = concat!(stringify!($libpath), ".dll");

            pub fn reload() {
                let path = unsafe {
                    let delete_old = DYLIB.is_some();

                    // Drop the old
                    DYLIB = None;

                    // Clean up the old
                    if delete_old {
                        let old_path = format!("{}{}", DYLIB_PATH, VERSION - 1);
                        std::fs::remove_file(&old_path).expect("Failed to delete old dylib");
                    }

                    // Create the new
                    let new_path = format!("{}{}", DYLIB_PATH, VERSION);
                    std::fs::copy(DYLIB_PATH, &new_path).expect("Failed to copy new dylib");
                    new_path
                };
                
                // Load new version
                unsafe {
                    VERSION += 1;
                    DYLIB = Some(Library::new(&path).expect("Failed to load dylib"))
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
                unsafe {
                    let symbol: Symbol<extern "C" fn($($argtype),*) $(-> $returntype)?> =
                        lib.get(stringify!($fnname).as_bytes()).expect("Failed to get symbol from dylib");
                    symbol($($argname),*)
                }
            }
            )*
        }
    }
}
