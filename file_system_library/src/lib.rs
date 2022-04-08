
use std::path::PathBuf;
use std::rc::Rc;
use std::ffi::{OsString, OsStr};
use std::fmt::Display
;
use libloading::Library;
use ilibrary::{ILibrarySource, ILibrary, ILibraryGenerator};

type Error = Box<dyn std::error::Error>;

pub struct  FileSystemLibrary {
   path: OsString,
   lib: Option<Library>,
   source_identifier: OsString,
}

impl FileSystemLibrary {
   pub fn new(path: OsString, source_identifier: OsString) -> Self {
       Self {
           lib: None,
           path,
           source_identifier,
       }
   }

   fn build(path: &OsString) -> Result<Library, Error> {
       unsafe {
           Ok(Library::new(path)?)
       }
   }
}

impl ILibrary for FileSystemLibrary {
   fn load(&mut self) -> Result<(), Error> {
       self.lib = Some(Self::build(&self.path)?);

       Ok(())
   }

   fn check(&self) -> Result<(), Error> {
       Self::build(&self.path)?;

       Ok(())
   }

   fn identifier(&self) -> &OsStr {
       &self.source_identifier
   }
}

impl Display for FileSystemLibrary {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{:p} ({:?})", &self, self.path)
   }
}


pub struct FileSystemLibraryGenerator
{
   path: OsString,
}

impl FileSystemLibraryGenerator {
   pub fn new(path: OsString) -> Self {
      Self {
         path,
      }
   }
}

impl ILibraryGenerator for FileSystemLibraryGenerator {
   fn generate(&self) -> Vec<Box<dyn ILibrary>> {
      vec![Box::new(
         FileSystemLibrary::new(
            self.path.clone(),
            self.path.clone(),
         )
      )]
   }
}


pub struct FileSystemLibrarySource
{
   generators: Vec<Box<dyn ILibraryGenerator>>,
}

impl FileSystemLibrarySource
{
    pub fn new() -> Self {
        Self {
            generators: Vec::new(),
        }
    }

    pub fn from_file(mut self, path: OsString) -> Self {
        let generator = FileSystemLibraryGenerator::new(path);

        self.generators.push(Box::new(generator));

        self
    }

    pub fn from_dir(self, dir: OsString) -> DirSourceBuilder {
        DirSourceBuilder::new(self, dir)
    }

    fn add_source(&mut self, source: Box<dyn ILibraryGenerator>) {
        self.generators.push(source);
    }
}

impl ILibrarySource for FileSystemLibrarySource {
   fn generate(&self) -> Vec<Box<dyn ILibrary>> {
      self.generators.iter().flat_map(|x| x.generate()).collect()
   }
}


pub struct DirSource
{
   dir: OsString,
   filter: Option<Rc<dyn Fn(&PathBuf) -> bool>>,
   action_if_error: Option<Rc<dyn Fn(std::io::Error)>>,
}

impl ILibraryGenerator for DirSource {
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
            FileSystemLibraryGenerator::new(path.into_os_string()).generate()
         })
         .collect();

         files
   }
}


pub struct DirSourceBuilder
{
   dir: OsString,
   source: FileSystemLibrarySource,
   filter: Option<Rc<dyn Fn(&PathBuf) -> bool>>,
   action_if_error: Option<Rc<dyn Fn(std::io::Error)>>,
}

impl DirSourceBuilder
{
   pub fn new(source: FileSystemLibrarySource, dir: OsString) -> Self {
      Self {
         dir,
         source,
         filter: None,
         action_if_error: None,
      }
   }

   pub fn filter_by(mut self, action: Rc<dyn Fn(&PathBuf) -> bool>) -> Self
   {
      self.filter = Some(action);
      
      self
   }

   pub fn if_error_read(mut self, action: Rc<dyn Fn(std::io::Error)>) -> Self {
      self.action_if_error = Some(action);

      self
   }

   pub fn read(mut self) -> FileSystemLibrarySource {
      let generator = DirSource {
         dir: self.dir,
         filter: self.filter,
         action_if_error: self.action_if_error,
      };

      self.source.add_source(Box::new(generator));

      self.source
   }
}
