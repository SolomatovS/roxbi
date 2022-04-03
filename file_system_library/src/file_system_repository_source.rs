#![allow(dead_code)]

pub use std::ffi::OsString;
pub use std::path::PathBuf;
use std::{rc::Rc, cell::RefCell};

use crate::file_system_library::FileSystemLibrary;
use ilibrary::{ILibrarySource, ILibrary, ILibraryGenerator};

type Error = Box<dyn std::error::Error>;
type FError = Box<dyn Fn(Error)>;

pub struct FileSystemRepositorySourceItem
{
   path: OsString,
}

impl FileSystemRepositorySourceItem {
   pub fn new(path: OsString) -> Self {
      Self {
         path,
      }
   }
}

impl ILibraryGenerator for FileSystemRepositorySourceItem {
   fn generate(&self) -> Vec<Box<dyn ILibrary>> {
      vec![Box::new(
         FileSystemLibrary::new(
            self.path.clone(),
            self.path.clone(),
         )
      )]
   }
}

pub struct ReadDirSource
{
   dir: OsString,
   filter: Option<Rc<dyn Fn(&PathBuf) -> bool>>,
   action_if_error: Option<Rc<dyn Fn(std::io::Error)>>,
}

pub struct ReadDirSourceBuilder
{
   dir: OsString,
   source: FileSystemRepositorySource,
   filter: Option<Rc<dyn Fn(&PathBuf) -> bool>>,
   action_if_error: Option<Rc<dyn Fn(std::io::Error)>>,
}

impl ReadDirSourceBuilder
{
   pub fn new(source: FileSystemRepositorySource, dir: OsString) -> Self {
      Self {
         dir,
         source,
         filter: None,
         action_if_error: None,
      }
   }

   pub fn filtration(mut self, action: Rc<dyn Fn(&PathBuf) -> bool>) -> Self
   {
      self.filter = Some(action);
      
      self
   }

   pub fn if_error(mut self, action: Rc<dyn Fn(std::io::Error)>) -> Self {
      self.action_if_error = Some(action);

      self
   }

   pub fn read(mut self) -> FileSystemRepositorySource {
      let generator = ReadDirSource {
         dir: self.dir,
         filter: self.filter,
         action_if_error: self.action_if_error,
      };

      self.source.generators.push(Box::new(generator));

      self.source
   }
}

impl ILibraryGenerator for ReadDirSource {
   fn generate(&self) -> Vec<Box<dyn ILibrary>> {
      let result = vec![];

      let dir = match std::fs::read_dir(self.dir.clone()) {
         Ok(dir) => dir,
         Err(e) => {
            if let Some(a) = &self.action_if_error {
               a(e);
            }
            
            return result;
      }};

      let filter = match &self.filter {
         Some(filter) => filter,
         None => return result,
      };

      let action = &self.action_if_error;
      let files = dir.into_iter()
         .filter_map(|path| {
            match path {
               Ok(path) => Some(path.path()),
               Err(e) => {
                  if let Some(action) = action {
                     action(e);
                  }
                  
                  None
               }
            }
         })
         .filter(|path| path.is_file())
         .filter(|p| {
            (*filter)(p)
         })
         .flat_map(|path| {
            FileSystemRepositorySourceItem::new(path.into_os_string()).generate()
         })
         .collect();

         files
   }
}


pub struct FileSystemRepositorySource
{
   generators: Vec<Box<dyn ILibraryGenerator>>,
}

impl FileSystemRepositorySource
{
   pub fn new() -> Self {
      Self {
         generators: Vec::new(),
      }
   }

   pub fn add_file_path(mut self, path: OsString) -> Self {
      let generator = FileSystemRepositorySourceItem::new(path);

      self.generators.push(Box::new(generator));

      self
   }

   pub fn from_dir(self, dir: OsString) -> ReadDirSourceBuilder {
      ReadDirSourceBuilder::new(self, dir)
   }
}

impl ILibrarySource for FileSystemRepositorySource {
   fn generate(&self) -> Vec<Box<dyn ILibrary>> {
      self.generators.iter().flat_map(|x| x.generate()).collect()
   }
}
