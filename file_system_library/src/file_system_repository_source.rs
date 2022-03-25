#![allow(dead_code)]

pub use std::ffi::OsString;
pub use std::path::PathBuf;

use crate::file_system_library::FileSystemLibrary;
use ilibrary::{ILibrarySource, ILibrary, ILibraryBuilder};

type Error = Box<dyn std::error::Error>;
type FError = Box<dyn Fn(Error)>;

pub struct FileSystemRepositorySourceItem
{
   path: OsString,
   action_if_build_error: FError
}

impl FileSystemRepositorySourceItem {
   pub fn new(path: OsString, action_if_build_error: FError) -> Self {
      Self {
         path,
         action_if_build_error,
      }
   }
}

impl ILibraryBuilder for FileSystemRepositorySourceItem {
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
         path, 
         Box::new(|_| {}),
      ));

      self
   }

   pub fn add_file_path_and_action_if_build_error(mut self, path: OsString, action_if_build_error: FError) -> Self
   {
      self.path.push(FileSystemRepositorySourceItem::new(
         path, 
         action_if_build_error,
      ));

      self
   }

   fn add_files_from_dir_common<F, K>(mut self, dir: OsString, file_filter: F, file_maker: K) -> Result<Self, Error>
   where
      F: Fn(&PathBuf) -> bool,
      K: Fn(PathBuf) -> FileSystemRepositorySourceItem,
   {
      let dir = match std::fs::read_dir(dir) {
         Ok(dir) => dir,
         Err(e) => {
            println!("{:?}", e);
            return Err(Box::new(e));
      }};

      dir.into_iter()
         .filter_map(|path| {
            match path {
               Ok(p) => Some(p.path()),
               Err(e) => {
                  println!("{:?}", e);
                  None
               }
            }
         })
         .filter(|path| path.is_file())
         .filter(file_filter)
         .for_each(|x| {
            self.path.push(file_maker(x));
         });
      
      Ok(self)
   }

   fn filter_by_extension(extension: OsString) -> impl Fn(&PathBuf) -> bool
   {
      move |x| {
         match x.extension() {
            Some(f) => f == &extension,
            None => false,
        }
      }
   }

   pub fn add_files_from_dir<K>(self, dir: OsString, extension: OsString, file_maker: K) -> Result<Self, Error>
   where
      K: Fn(PathBuf) -> FileSystemRepositorySourceItem,
   {
      self.add_files_from_dir_common(
         dir,
         Self::filter_by_extension(extension),
         file_maker
      )
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
