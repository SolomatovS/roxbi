#![allow(dead_code)]

pub use std::ffi::OsString;
use std::{fs::ReadDir, rc::Rc};
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


struct ReadDirSource
{
   dir: OsString,
   source: FileSystemRepositorySource,
   filter: Vec<Rc<Box<dyn FnMut(&PathBuf) -> bool>>>,
   action_if_error: Option<Box<dyn Fn(std::io::Error)>>,
   maker: Box<dyn Fn(PathBuf) -> FileSystemRepositorySourceItem>,
}

impl ReadDirSource
{
   fn new(source: FileSystemRepositorySource, dir: OsString, maker: Box<dyn Fn(PathBuf) -> FileSystemRepositorySourceItem>) -> Self {
      Self {
         dir,
         source,
         filter: vec![],
         action_if_error: None,
         maker,
      }
   }

   fn filter_by_extension(mut self, extension: OsString) -> Self {
      let lambda = move |x: &PathBuf| {
         match x.extension() {
            Some(f) => f == extension,
            None => false,
      }};

      self.filter.push(Rc::new(Box::new(lambda) as Box<dyn FnMut(&PathBuf) -> bool>));
      
      self
   }

   fn if_error_then(mut self, action: Box<dyn Fn(std::io::Error)>) -> Self {
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

      let files = dir.into_iter()
         .filter_map(|path| {
            match path {
               Ok(p) => Some(p.path()),
               Err(e) => {
                  if let Some(a) = self.action_if_error {
                     a(e);
                  }
                  
                  None
               }
            }
         })
         .filter(|path| path.is_file());

      self.filter.into_iter().for_each(|f| {
         files.filter(*f);
      });

      files.for_each(|x| {
         self.source.path.push((self.maker)(x));
      });
      
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

impl FileSystemRepositorySource
{
   pub fn new() -> Self {
      Self {
         path: Vec::new(),
      }
   }

   pub fn add_file_path(mut self, path: OsString) -> Self {
      self.path.push(FileSystemRepositorySourceItem::new(
         path
      ));

      self
   }

   fn read_dir<'a, E>(self, dir: OsString, maker: E) -> ReadDirSource
   where
      E: Fn(PathBuf) -> FileSystemRepositorySourceItem,
   {
      ReadDirSource::new(self, dir, Box::new(maker))
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
