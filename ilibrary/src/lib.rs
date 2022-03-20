use std::array::IntoIter;

type Error = Box<dyn std::error::Error>;

pub trait ILibrary {
}

pub trait IRepositoryLibrarySource : IntoIterator {

}

pub struct RepositoryLibrary {
    pub source_library: Vec<Box<dyn IRepositoryLibrarySource<Item = Box<dyn ILibrary>, IntoIter = Box<dyn Iterator<Item = Box<dyn ILibrary>>>>>>,
    pub builded_library: Vec<Box<dyn ILibrary>>,
 }
 
 impl RepositoryLibrary {
    pub fn builder() -> RepositoryLibraryBuilder {
       RepositoryLibraryBuilder {
          source: vec![]
       }
    }
 }

pub struct RepositoryLibraryBuilder
{
   source: Vec<Box<dyn IRepositoryLibrarySource<Item = Box<dyn ILibrary>, IntoIter = Box<dyn Iterator<Item = Box<dyn ILibrary>>>>>>
}


impl RepositoryLibraryBuilder {
    pub fn build(self) -> RepositoryLibrary {
        let libs = self.source.iter().flat_map(|x| x.iter());

       RepositoryLibrary {
        source_library: self.source,
        builded_library: libs.iter().collect(),
       }
    }
 
     pub fn add_source(mut self, source: Box<dyn IRepositoryLibrarySource<Item = Box<dyn ILibrary>, IntoIter = Box<dyn Iterator<Item = Box<dyn ILibrary>>>>>) -> Self {
       self.source.push(source);
 
       self
    }
 }
 