use dymod::dymod;

dymod! {
    #[path = "./subcrate/src/lib.rs"]
    pub mod subcrate {
        fn count_sheep(sheep: u32) -> &'static str;
    }
}

fn main() {
    #[cfg(debug_assertions)]
    const MESSAGE: &str = "
You are running in debug mode.
Make changes to subcrate/src/lib.rs
Then run `cargo build` in the subcrate directory.
You should see your changes apply while this code runs:";

    #[cfg(not(debug_assertions))]
    const MESSAGE: &str = "
You are running in release mode.
The `subcrate` module has been statically linked.
Any changes made to subcrate/src/lib.rs will not apply until this program is recompiled.";

    let path = std::env::current_dir().expect("werwer");
    println!("The current directory is {}", path.display());
    println!("{}", MESSAGE);
    println!("{}", env!("CARGO_MANIFEST_DIR"));
    println!("\nPress Ctrl+C to quit.\n");

    loop {
        let num_sheep = 3;
        let message = subcrate::count_sheep(num_sheep);
        println!("There are '{}' sheep.", message);
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}
