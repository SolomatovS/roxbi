#![allow(dead_code)]

pub use std::ffi::OsString;
use std::{fs::ReadDir, rc::Rc, ffi::OsStr, cell::RefCell, borrow::{Borrow, BorrowMut}};
pub use std::path::PathBuf;

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
   fn generate(&self) -> Box<dyn ILibrary>
   {
      Box::new(
         FileSystemLibrary::new(
            self.path.clone(),
            self.path.clone(),
         )
      )
   }
}

/*
pub struct ReadDirSource
{
   dir: OsString,
   source: FileSystemRepositorySource,
   filter: Option<Rc<RefCell<dyn FnMut(&PathBuf) -> bool>>>,
   action_if_error: Option<Rc<dyn Fn(std::io::Error)>>,
}

impl ReadDirSource
{
   fn new(source: FileSystemRepositorySource, dir: OsString) -> Self {
      Self {
         dir,
         source,
         filter: None,
         action_if_error: None,
      }
   }

   fn set_filter(mut self, callback: Rc<RefCell<dyn FnMut(&PathBuf) -> bool>>) -> Self
   {
      self.filter = Some(callback);
      
      self
   }

   fn if_error_then(mut self, action: Rc<dyn Fn(std::io::Error)>) -> Self {
      self.action_if_error = Some(action);

      self
   }

   fn build(mut self) -> FileSystemRepositorySource
   {
      let dir = match std::fs::read_dir(self.dir.clone()) {
         Ok(dir) => dir,
         Err(e) => {
            if let Some(a) = self.action_if_error {
               a(e);
            }
            
            return self.source
      }};

      let filter = match self.filter {
         Some(filter) => filter,
         None => return self.source,
      };

      let mut filter = (*filter).borrow_mut();
      
      let action = &self.action_if_error;
      let files: Vec<PathBuf> = dir.into_iter()
         .filter_map(|path| {
            match path {
               Ok(path) => Some(path.path()),
               Err(e) => {
                  if let Some(a) = action {
                     a(e);
                  }
                  
                  None
               }
            }
         })
         .filter(|path| path.is_file())
         .filter(|p| -> bool {
            (*filter)(p)
         })
         .collect();

      for path in files {
            self.source.add_file_path(path.into_os_string());
      }
      
      self.source
   }
}
*/


















pub struct ReadDirSource<'a>
{
   dir: &'a OsStr,
   source: &'a FileSystemRepositorySource,
   filter: Option<Box<dyn FnMut(&PathBuf) -> bool>>,
   action_if_error: Option<Box<dyn Fn(std::io::Error)>>,
}

impl<'a> ReadDirSource<'a>
{
   fn new(source: &'a FileSystemRepositorySource, dir: &'a OsStr) -> Self {
      Self {
         dir,
         source,
         filter: None,
         action_if_error: None,
      }
   }
/*
   fn set_filter(mut self, callback: Rc<RefCell<dyn FnMut(&PathBuf) -> bool>>) -> Self
   {
      self.filter = Some(callback);
      
      self
   }
*/
   fn set_filter<F>(&mut self, callback: &'a F) -> &mut Self
   where F: FnMut(&PathBuf) -> bool
   {
      /*
      let f = Rc::new(RefCell::new(callback));
      self.filter = Some(f);
      
      self
      */
      self
   }

   fn if_error_then<F>(&mut self, callback: &'a F) -> &mut Self
   where F: Fn(std::io::Error)
   {
      //self.action_if_error = Some(action);

      self
   }

   fn build(mut self) -> &'a FileSystemRepositorySource
   {
      let dir = match std::fs::read_dir(self.dir.clone()) {
         Ok(dir) => dir,
         Err(e) => {
            if let Some(a) = self.action_if_error {
               a(e);
            }
            
            return self.source
      }};

      let mut filter = match self.filter {
         Some(filter) => filter,
         None => return self.source,
      };
      
      let action = &self.action_if_error;
      let files: Vec<PathBuf> = dir.into_iter()
         .filter_map(|path| {
            match path {
               Ok(path) => Some(path.path()),
               Err(e) => {
                  if let Some(a) = action {
                     a(e);
                  }
                  
                  None
               }
            }
         })
         .filter(|path| path.is_file())
         .filter(|p| -> bool {
            (*filter)(p)
         })
         .collect();

      /*
      for path in files {
            &(self.source).add_file_path(path.clone().into_os_string());
      }
      */
      
      self.source
   }
/*
   pub fn filter_by_extension<K, E>(mut self, extension: OsString, file_maker: K, action_if_error: E) -> FileSystemRepositorySource
   where
      K: Fn(PathBuf) -> FileSystemRepositorySourceItem,
      E: Fn(std::io::Error),
   {
      self.add_files_common (
         Self::filter_by_extension(extension),
         file_maker,
         action_if_error
      )
   }
*/
}






pub struct FileSystemRepositorySource
{
   path: Vec<FileSystemRepositorySourceItem>,
}

impl<'a> FileSystemRepositorySource
{
   pub fn new() -> Self {
      Self {
         path: Vec::new(),
      }
   }

   pub fn add_file_path(&mut self, path: OsString) -> &mut Self {();
      self.path.push(FileSystemRepositorySourceItem::new(
         path
      ));

      self
   }

   
   pub fn read_dir(&'a mut self, dir: &'a OsStr) -> ReadDirSource<'a>
   {
      ReadDirSource::new(self, dir)
   }
   
   fn add_files_from_dir_common<F, K, E>(mut self, dir: ReadDir, file_filter: F, file_maker: K, action_if_error: E) -> Self
   where
      F: Fn(&PathBuf) -> bool,
      K: Fn(PathBuf) -> FileSystemRepositorySourceItem,
      E: Fn(std::io::Error),
   {
      dir.into_iter()
         .filter_map(|path| {
            match path {
               Ok(p) => Some(p.path()),
               Err(e) => {
                  action_if_error(e);
                  None
               }
            }
         })
         .filter(|path| path.is_file())
         .filter(file_filter)
         .for_each(|x| {
            self.path.push(file_maker(x));
         });
      
      self
   }
}

impl ILibrarySource for FileSystemRepositorySource {
   fn generate(&self) -> Vec<Box<dyn ILibrary>> {
      self.path.iter()
         .map(|x| {
            x.generate()
         }).collect()
   }
}
