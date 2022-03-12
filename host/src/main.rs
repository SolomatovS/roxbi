mod helpers;

use isay_hello::ISayHelloService;
use std::ffi::OsStr;
use std::boxed::Box;
use std::fs;

use crate::helpers::DynamicLibraryManager;
use crate::helpers::*;

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

        // всё, что не является файлом пропускаем
        if path.is_file() == false {
            continue;
        }

        match path.extension() {
            Some(extension) =>
                if extension != lib_extensions {continue},
            None => continue,
        }

        println!("try load {:?}", &path);
        if let Err(e) = loader.add_library(path.into_os_string()) {
            println!("{:?}", e);
        }
    }

    loader.find_symbol::<extern "Rust" fn() -> Box<dyn ISayHelloService>>(OsStr::new("new"))
        .iter()
        .map(|x| x())
        .for_each(|x| x.say_hello())
    ;

    let repo = DLibRepository::new();
    let source = 
    //repo.add(s: Box<dyn IDLibRepositorySource>)
}