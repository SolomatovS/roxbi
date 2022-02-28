use isay_hello::ISayHelloService;

fn main() {
    let path = "/Users/solomatovs/Documents/GitHub/roxbi/plugins/target/debug/libsay_hello_console.dylib";

    let service = new_plugin::<dyn ISayHelloService>(path).expect("lib doesn't load");
    service.say_hello();
}

fn new_plugin<T: ?Sized>(path: &str) -> Result<Box<T>, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new(path)?;
        let new: libloading::Symbol<unsafe extern "Rust" fn() -> Box<T>> = lib.get(b"new")?;
        
        Ok(new())
    }
}