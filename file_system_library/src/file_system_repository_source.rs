#![allow(dead_code)]

use std::ffi::OsString;

use crate::file_system_library::FileSystemLibrary;

type Error = Box<dyn std::error::Error>;

pub struct FileSystemRepositorySource {
   directory_path: Vec<OsString>,
   path: Vec<OsString>,
}

impl FileSystemRepositorySource {
   pub fn new() -> Self {
      Self {
         directory_path: Vec::new(),
         path: Vec::new(),
      }
   }

   pub fn add_directory(&mut self, dir: OsString) {
      self.directory_path.push(dir);
   }

   //fn search_dlib_files(dir: &OsString, extension: &OsString) -> Result<Vec<OsString>, Error> {
   //fn search_dlib_files(dir: &OsString, extension: OsString) -> Result<impl Iterator<Item=OsString>, Error> {
   fn search_dlib_files<'a, F>(dir: &'a OsString, filter: F) -> Result<impl Iterator<Item=OsString>, Error>
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