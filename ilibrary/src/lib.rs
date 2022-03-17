type Error = Box<dyn std::error::Error>;

pub trait ILibrary {
    type Item;

    fn get(&self) -> &Self::Item;
}

pub trait IDLibRepositorySource {
    type Item;

    fn build<'a, F>(&mut self, action_for_error: F) -> Vec<Box<dyn ILibrary<Item=Self::Item>>>
    where
       F: Fn(Error);
}
