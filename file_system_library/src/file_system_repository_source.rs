#![allow(dead_code)]

use std::ffi::{OsString, OsStr};
use std::path::PathBuf;

use crate::file_system_library::FileSystemLibrary;

type Error = Box<dyn std::error::Error>;

pub struct FileSystemRepositorySource {
   path: Vec<(OsString, bool)>,
}

impl FileSystemRepositorySource {
   pub fn new() -> Self {
      Self {
         path: Vec::new(),
      }
   }

   pub fn add_file_path(&mut self, path: OsString, required_for_load: bool) -> &mut Self {
      self.path.push((path, required_for_load));

      self
   }

   fn add_files_from_dir_common<F, K>(&mut self, dir: OsString, file_filter: F, file_maker: K) -> Result<&mut Self, Error>
   where
      F: Fn(&PathBuf) -> bool,
      K: Fn(PathBuf) -> (OsString, bool)
   {
      let dir = match std::fs::read_dir(dir) {
         Ok(dir) => dir,
         Err(e) => {
            //println!("{:?}", dir);
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
      K: Fn(PathBuf) -> (OsString, bool)
   {
      self.add_files_from_dir_common(dir, Self::filter_by_extension(extension), file_maker)
   }

   //fn search_dlib_files(dir: &OsString, extension: &OsString) -> Result<Vec<OsString>, Error> {
   //fn search_dlib_files(dir: &OsString, extension: OsString) -> Result<impl Iterator<Item=OsString>, Error> {
   pub fn search_dlib_files<'a, F>(dir: &'a OsString, filter: F) -> Result<impl Iterator<Item=OsString>, Error>
   where
      F: FnMut(&std::path::PathBuf) -> bool
   {
      let dir = match std::fs::read_dir(dir) {
         Ok(dir) => dir,
         Err(e) => {
            println!("{:?}", dir);
            println!("{:?}", e);
            return Err(Box::new(e));
      }};

      let paths = dir.into_iter()
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
/*
         .filter(move |path| match path.extension() {
            Some(p) => p == extension,
            None => false
         })
*/
         .filter(filter)
         .map(|path| path.into_os_string())
         //.collect()
      ;
      
      Ok(paths)
   }

   pub fn build(&mut self) -> Vec<FileSystemLibrary> {
      self.path.iter()
         .filter_map(|x| Self::make_lib(x.0.to_os_string()))
         .collect()
   }

   fn make_lib(path: OsString) -> Option<FileSystemLibrary> {
      match FileSystemLibrary::new(path.to_os_string()) {
         Ok(lib) => Some(lib),
         Err(e) => {
            println!("{:?}", &e);
            None
         }
      }
   }

   /*
   pub fn build(&mut self, extension: &OsString) -> Vec<FileSystemLibrary> {
      let lambda = |path: &std::path::PathBuf| match path.extension() {
         Some(x) => x == extension,
         None => false
      };

      self.directory_path.iter()
         // достаю пути к библиотекам которые расположены в указанной папке dir
         //.filter_map(|dir| match Self::search_dlib_files(dir, extension) {
         .filter_map(|dir| match Self::search_dlib_files(dir, lambda) {
            Ok(x) => Some(x),
            Err(_) => None,
         })
         // объединяю результаты сканирования нескольких папок в один массив
         .flat_map(|x| x)
         // загружаю библиотеки по указанным путям
         .filter_map(|x| {
            match FileSystemLibrary::new(x) {
               Ok(lib) => Some(lib),
               Err(e) => {
                  println!("{:?}", &e);
                  None
               }
            }
         })
         .collect()
   }
   */
}


/*
impl FileSystemRepositorySource {
   fn values(&self) {
      self.library.iter()
   }
}


impl IntoIterator for FileSystemRepositorySource {
   type IntoIter = impl Iterator<Item=ILibrary>;
   fn into_iter(self) -> Self::IntoIter;{

   }
}
*/

/*
    impl FileSystemDinamicLibrarySource {
    pub fn new() -> Self {
    Self {
    library: Vec::new()
    }
     }
    
    pub fn add_from_path(&mut self, path: OsString) -> Result<(), Box<dyn Error>>
     {
    let lib: Library;
    unsafe {
    lib = Library::new(path.as_os_str())?;
     }
    
    self.library.push(FileSystemLibrary::new(path, lib));
    
    Ok(())
     }
    }
    
    impl IDLibRepositorySource for FileSystemDinamicLibrarySource {
    }
    
    impl Iterator for FileSystemDinamicLibrarySource {
    type Item = Box<dyn ILibrary>;
    
    fn next(&mut self) -> Option<Self::Item> {
    todo!()
     }
    
     fn get_libs(&self) -> impl Iterator<Item = &dyn ILibrary> {
    
     let sdf = self.library.iter()
     .map(|x| x as &dyn ILibrary)
     ;
    
     sdf
     }
    }
*/