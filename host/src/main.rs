mod helpers;

use std::{ffi::{OsString}, path::PathBuf};
use std::{rc::Rc};
use file_system_library::{FileSystemLibrarySource};
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

    let source = FileSystemLibrarySource::new()
        .from_file(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib"))
        .from_file(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib1"))        
        .from_dir(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/"))
            .filter_by(clone_filter)
            .if_error_read(clone_error)
            .read()
        .from_file(OsString::from("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib1"))
    ;

    let mut libraries = source.generate();

    libraries.iter_mut().for_each(|f| {
        if let Err(e) = f.load() {
            println!("{:?}", &e);
        } else {
            println!("success loaded: {}", &f);
        }
    });
}