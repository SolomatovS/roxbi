#![allow(dead_code)]

mod helpers;

use std::{ffi::{OsString}, path::PathBuf};
use std::{rc::Rc, cell::RefCell};
use file_system_library::file_system_repository_source::{FileSystemRepositorySource};
use ilibrary::ILibrarySource;


fn main() {
    let filter = Rc::new(|path: &PathBuf| {
        if let Some(extension) = path.extension() {
            return extension == "dylib";
        }

        false
    });

    let if_error = Rc::new(|error| {
        println!("{:?}", &error);
    });

    let clone_filter = Rc::clone(&filter);
    let clone_error = Rc::clone(&if_error);

    let source = FileSystemRepositorySource::new()
        .add_file_path(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib"))
        .add_file_path(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib1"))        
        .from_dir(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/"))
            .filtration(clone_filter)
            .if_error(clone_error)
            .read()
        .add_file_path(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib1"))
    ;

    let libraries = source.generate();

    libraries.iter().for_each(|f| {
        match f.check() {
            Ok(f) => f,
            Err(e) => println!("{:?}", &e)
        }
    });
}