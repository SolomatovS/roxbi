mod helpers;

use isay_hello::ISayHelloService;
use std::{ffi::OsStr, path};
use std::error::Error;
use std::boxed::Box;
use std::rc::Rc;
use std::fs;
use libloading::{Library, Symbol};

use crate::helpers::DynamicLibraryManager;

fn main() {
    let mut loader = DynamicLibraryManager::new();
    let lib_path = OsStr::new("/Users/solomatovs/Documents/GitHub/roxbi/plugins/target/debug");
    let lib_extensions = OsStr::new("dylib");

    let dir = match fs::read_dir(lib_path) {Ok(dir) => dir, Err(e) => {
        println!("lib path {:?} doesn't exist, error: {:?}", lib_path, e);
        return;
    }};

    // поочередно проверяем каждый файл в указанной директории
    for path in dir {
        let path = match path {
            Ok(p) => p,
            Err(e) => {
                println!("{:?}", lib_path);
                println!("{:?}", e);
                return;
        }};

        let path = path.path();

        // всё, что не является файлов пропускаем
        if path.is_file() == false {
            continue;
        }

        match path.extension() {
            Some(extension) =>
                if extension != lib_extensions {continue},
            None => continue,
        }
        
        //loader.add_library(Rc::new(path.into_os_string()));

        loader.add_library(path.into_os_string());
        /*
        let service = match get_service::<dyn ISayHelloService>(path.as_os_str(), b"new") {
            Ok(service) => service,
            Err(e) => {
                println!("{:?} doesn't load", path);
                println!("{:?}", e);
                continue;
            },
        };
        
        service.say_hello();
        */
    }
}

fn get_service<T: ?Sized>(path: &OsStr, symbol: &[u8]) -> Result<Box<T>, Box<dyn Error>> {
    unsafe {
        let lib = Library::new(path)?;
        let new: Symbol<extern "Rust" fn() -> Box<T>> = lib.get(symbol)?;
        
        Ok(new())
    }
}
