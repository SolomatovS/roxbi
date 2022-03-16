#![allow(dead_code)]

use std::ffi::OsString;
use std::path::PathBuf;

use crate::file_system_library::FileSystemLibrary;

type Error = Box<dyn std::error::Error>;

pub struct FileSystemRepositorySource {
   path: Vec<OsString>,
}

impl FileSystemRepositorySource {
   pub fn new() -> Self {
      Self {
         path: Vec::new(),
      }
   }

   pub fn add_file_path(&mut self, path: OsString) -> &mut Self {
      self.path.push(path);

      self
   }

   fn add_files_from_dir_common<F, K>(&mut self, dir: OsString, file_filter: F, file_maker: K) -> Result<&mut Self, Error>
   where
      F: Fn(&PathBuf) -> bool,
      K: Fn(PathBuf) -> OsString,
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

   pub fn add_files_from_dir<K>(&mut self, dir: OsString, extension: OsString, file_maker: K) -> Result<&mut Self, Error>
   where
      K: Fn(PathBuf) -> OsString,
   {
      self.add_files_from_dir_common(dir, Self::filter_by_extension(extension), file_maker)
   }

   pub fn build<'a, F>(&mut self, action_for_error: F) -> Vec<FileSystemLibrary>
   where
      F: Fn(Error)
   {
      let col = self.path.iter()
         .filter_map(|x| {
            match FileSystemLibrary::new(x.to_os_string()) {
               Ok(x) => Some(x),
               Err(e) => {
                  action_for_error(e);
                  None
               },
            }
         })
         .collect();
      
      col
   }
}
