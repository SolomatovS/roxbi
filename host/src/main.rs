mod helpers;

use fslib::FileSystemSource;
use lib::ILibrarySource;
use std::error::Error;
use std::path::PathBuf;

fn main() {
    let filter = |path: &PathBuf| {
        if let Some(extension) = path.extension() {
            return extension == "dylib";
        }

        false
    };
    
    let if_error = |error: &Box<dyn Error>| {
        println!("{:?}", &error);
    };

    let source = FileSystemSource::new()
        .from_file("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib".into())
        .from_file("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib1".into())
        .from_dir("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug".into())
            .filter_by(filter)
            .if_error_read(if_error)
        .from_file("/Users/solomatovs/Documents/GitHub/roxbi/say_hello_console/target/debug/libsay_hello_console.dylib2".into())
        .build();

    let (files, errors) = source.generate();

    errors.iter().for_each(|e| println!("error: {:?}", &e));
    files.iter().for_each(|f| println!("success: {}", &f));
}
