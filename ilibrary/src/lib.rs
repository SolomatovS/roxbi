pub trait ILibrary {
    fn build(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn get(&self) -> Option<&libloading::Library>;
}
