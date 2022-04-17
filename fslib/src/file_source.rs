use std::ffi::OsString;
use std::error::Error;

use crate::DirSourceBuilder;

use super::file_system_source::*;
use super::file_system_library::*;


type TAction = Box<dyn Fn(&Box<dyn Error>)>;

pub struct FileSource {
    pub(super) path: OsString,
    pub(super) action_if_error: Option<TAction>,
}

impl FileSource {
    pub fn new(path: OsString) -> Self {
        Self {
            path,
            action_if_error: None,
        }
    }

    pub fn build_library(&self) -> Result<FileSystemLibrary, Box<dyn Error>> {
        let lib = FileSystemLibrary::new(
            self.path.clone(), 
            self.path.clone()
        );

        match lib {
            Ok(lib) => Ok(lib),
            Err(e) => {
                if let Some(a) = &self.action_if_error {
                    a(&e);
                }

                Err(e)
            }
        }
    }
}



pub struct FileSourceBuilder {
    pub(super) source: FileSystemSource,
    pub(super) path: OsString,
    pub(super) is_required: bool,
    pub(super) action_if_error: Option<TAction>,
}


impl FileSourceBuilder {
    pub fn new(source: FileSystemSource, path: OsString) -> Self {
        Self {
            source,
            path,
            is_required: true,
            action_if_error: None,
        }
    }

    pub fn is_required(mut self) -> Self {
        self.is_required = true;

        self
    }

    pub fn if_error_load(mut self, action: fn(&Box<dyn Error>)) -> Self {
        self.action_if_error = Some(Box::new(action));

        self
    }

    pub fn build(mut self) -> FileSystemSource {
        let source = FileSource {
            path: self.path,
            action_if_error: self.action_if_error,
        };

        self.source.source.push(FileSystemSourceEnum::File(source));

        self.source
    }

    pub fn from_file(self, path: OsString) -> FileSourceBuilder {
        self
            .build()
            .from_file(path)
    }

    pub fn from_dir(self, dir: OsString) -> DirSourceBuilder {
        self
            .build()
            .from_dir(dir)
    }
}
