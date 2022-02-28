use isay_hello::ISayHelloService;

fn main() {
    let path = "/Users/solomatovs/Documents/GitHub/roxbi/plugins/target/debug/libsay_hello_console.dylib";

    let service = new_say_hello_service(path).expect("lib doesn't load");
    service.say_hello();
    service.say_hello();
}

fn new_say_hello_service(path: &str) -> Result<Box<dyn ISayHelloService>, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new(path)?;
        let new_service: libloading::Symbol<unsafe extern "Rust" fn() -> Box<dyn ISayHelloService>> = lib.get(b"new_service")?;
        
        Ok(new_service())
    }
}