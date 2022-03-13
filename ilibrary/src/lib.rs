pub trait ILibrary {
    type Item;

    fn build(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn get(&self) -> Option<&Self::Item>;
}
