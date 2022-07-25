use std::fmt; 
use std::io;
use std::path::Path;
use std::time::SystemTime;
use super::{Library, Error};

#[derive(Debug)]
pub enum DymodError {
  IOError(io::Error, String),
  LibloadingError(Error),
  DymodNonInitialized,
  SymbolNotFound(Error, String),
  PoisonError,
}

impl fmt::Display for DymodError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

#[derive(Debug)]
pub struct DymodSource {
  version: usize,
  modified_time: std::time::SystemTime,
  source_path: String,
  lib_path: String,
  lib: Library,
  manual_reload_needed: bool,
}


fn get_modified_date(file_path: &str) -> Result<SystemTime, DymodError> {
  let metadata = match std::fs::metadata(&file_path) {
    Err(e) => return Err(DymodError::IOError(e, String::from(format!("error getting metadata from {} file", file_path)))),
    Ok(metadata) => metadata,
  };

  let modified_time = match metadata.modified() {
    Err(e) => return Err(DymodError::IOError(e, String::from(format!("failed to get modified time of {} file", file_path)))),
    Ok(x) => x,
  };

  Ok(modified_time)
}

fn create_folder_for_file(file_path: &Path) -> Result<(), DymodError> {
  let folfer = file_path.parent().unwrap();
  
  match std::fs::create_dir_all(folfer) {
    Ok(_) => Ok(()),
    Err(e) => {
      Err(DymodError::IOError(e, format!("failed to create folder for file {:?}", file_path)))
    },
  }
}

impl DymodSource {
  pub fn reload_needed(&self) -> bool {
    // if manual reload turn on
    if self.manual_reload_needed {
      return true;
    }

    // if modified time changed
    match get_modified_date(&&self.source_path) {
      Ok(modified_time) => (modified_time != self.modified_time),
      Err(_) => true,
    }
  }

  pub fn version(&self) -> usize {
    self.version
  }

  pub fn source_file(&self) -> &str {
    &self.source_path
  }

  pub fn dest_file(&self) -> String {
    self.lib_path.clone()
  }

  pub fn create_new_version(&self) -> Result<DymodSource, DymodError> {
    let new_lib = DymodSource::new(&self.source_path,  self.version+1)?;

    Ok(new_lib)
  }

  pub fn get_lib_ref(&self) -> Result<&Library, DymodError> {
    return Ok(&self.lib)
  }

  pub fn reload_turn_on(&mut self) {
    self.manual_reload_needed = true
  }


  pub fn new(file_path: &str, version: usize) -> Result<Self, DymodError> {
    if !Path::new(&file_path).exists() {
      let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");

      return Err(DymodError::IOError(io_error, format!("source lib {} not found", &file_path)));
    }

    let copy_to = loop {
      let plugin = Path::new(file_path);
      let file_name = plugin.file_name().unwrap();
      let folder = plugin.parent().unwrap();

      let plugin = folder
        .join("plugin_temp")
        .join(file_name)
        .join(format!("{}", version))
      ;

      if plugin.exists() {
        if let Err(e) = std::fs::remove_file(&plugin) {
          return Err(DymodError::IOError(e, format!("file {:?} not remove", plugin)));
        }
      }

      if !plugin.exists() {
        break plugin;
      }
    };

    create_folder_for_file(&copy_to)?;
    
    if let Err(e) = std::fs::copy(&file_path, &copy_to) {
      return Err(DymodError::IOError(e, String::from(format!("copy failed {:?} -> {:?}", file_path, copy_to))))
    }

    match unsafe {Library::new(&copy_to)} {
      Ok(lib) => {
        Ok(DymodSource {
          modified_time: get_modified_date(&file_path)?,
          version: version,
          source_path: String::from(file_path),
          lib_path: String::from(copy_to.to_str().unwrap()),
          lib,
          manual_reload_needed: false,
        })
      },
      Err(e) => Err(DymodError::LibloadingError(e)),
    }
  }
}

