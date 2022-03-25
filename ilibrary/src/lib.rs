use std::{ffi::OsStr};
type Error = Box<dyn std::error::Error>;

pub trait ILibrary {
   fn reload(&mut self) -> Result<(), Error>;
   fn check(&self) -> Result<(), Error>;
   fn identifier(&self) -> &OsStr;
}

impl PartialEq for dyn ILibrary + '_  {
   fn eq(&self, other: &Self) -> bool {
      self.identifier() == other.identifier()
  }
}

impl Eq for dyn ILibrary + '_ {}

pub trait ILibraryGenerator {
   fn generate(&self) -> Box<dyn ILibrary>;
}

pub trait ILibrarySource {
   fn generate(&self) -> Vec<Box<dyn ILibrary>>;
}

pub struct RepositoryLibrary {
   sources: Vec<Box<dyn ILibrarySource>>,
   libs: Vec<Box<dyn ILibrary>>,
}

impl RepositoryLibrary {
   pub fn new() -> Self {
      Self {
         sources: vec![],
         libs: vec![],
      }
   }

   pub fn add_source<'a>(mut self, source: Box<dyn ILibrarySource>) -> Self {
      self.sources.push(source);

      self
   }

   pub fn build_missing_libs(mut self) {
      let new_libs: Vec<Box<dyn ILibrary>> = self.sources.iter()
         .flat_map(|x| x.generate())
         .collect();
      
      new_libs.into_iter()
         .for_each(|x| {
            let exist = self.libs.iter().any(|l| l.identifier() == x.identifier());
            if !exist {
               self.libs.push(x);
            }
         });
   }
}