#![allow(dead_code)]

mod helpers;

use std::ffi::OsString;

use file_system_library::file_system_repository_source::FileSystemRepositorySource;

fn main() {
    let mut file_source = FileSystemRepositorySource::new();

    let file_source = file_source
        .add_file_path(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib"), true)
    ;
    
    let libs = file_source.build();
    
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