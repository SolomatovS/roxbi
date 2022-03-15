pub trait ILibrary {
    type Item;

    fn get(&self) -> &Self::Item;
}

pub trait IDLibRepositorySource : Iterator {
    
}
