#![allow(dead_code)]

mod helpers;

use std::ffi::OsStr;
use std::env;

use file_system_library::file_system_repository_source::FileSystemRepositorySource;

fn main() {
    let mut file_source = FileSystemRepositorySource::new();

    let current_dir = env::current_dir();
    let current_exe = env::current_exe();
    let lib_path = OsStr::new("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug");
    let lib_extensions = OsStr::new("dylib");

    file_source.add_directory(lib_path.to_os_string());
    
    let libs = file_source.build(&lib_extensions.to_os_string());
    
    libs.iter().for_each(|x| println!("{}", x));
    /*
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
    */
}