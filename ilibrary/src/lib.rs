type Error = Box<dyn std::error::Error>;

pub trait ILibrary {
   fn id(&self) -> u32;
   fn reload(&mut self) -> Result<(), Error>;
   fn check(&self) -> Result<(), Error>;
}

impl PartialEq for dyn ILibrary + '_  {
   fn eq(&self, other: &Self) -> bool {
      self.id() == other.id()
  }
}
impl Eq for dyn ILibrary + '_ {}

pub trait ILibraryBuilder {
   fn build<'a>(self) -> Box<dyn Fn(u32) -> Box<dyn ILibrary>>;
}

pub trait IRepositoryLibrarySource : IntoIterator {

}

pub struct RepositoryLibrary {
   source: Vec<Box<dyn IRepositoryLibrarySource<Item = Box<dyn ILibrary>, IntoIter = Box<dyn Iterator<Item = Box<dyn ILibrary>>>>>>,
   cache: Vec<Box<dyn ILibrary>>,
}
 
impl RepositoryLibrary {
   pub fn new() -> Self {
      Self {
         source: vec![],
         cache: vec![],
      }
   }

   pub fn add_source(mut self, source: Box<dyn IRepositoryLibrarySource<Item = Box<dyn ILibrary>, IntoIter = Box<dyn Iterator<Item = Box<dyn ILibrary>>>>>) -> Self {
      self.source.push(source);

      self
   }
}



/*
pub struct RepositoryLibraryBuilder
{
   source: Vec<Box<dyn IRepositoryLibrarySource<Item = Box<dyn ILibrary>, IntoIter = Box<dyn Iterator<Item = Box<dyn ILibrary>>>>>>,
}


impl RepositoryLibraryBuilder {
   pub fn build(self) -> RepositoryLibrary {
      RepositoryLibrary {
         source_library: self.source,
         cache: vec![],
      }
   }

   pub fn add_source(mut self, source: Box<dyn IRepositoryLibrarySource<Item = Box<dyn ILibrary>, IntoIter = Box<dyn Iterator<Item = Box<dyn ILibrary>>>>>) -> Self {
      self.source.push(source);

      self
   }
}
*/
 