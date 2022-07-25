use std::ffi::OsString;
use std::path::PathBuf;
use std::error::Error;

use crate::FileSourceBuilder;

use iter_help::{IterHelp, LR};

use super::file_system_source::*;
use super::file_system_library::*;
use super::file_source::*;

type TFilter = Box<dyn Fn(&PathBuf) -> bool>;
type TAction = Box<dyn Fn(&Box<dyn Error>)>;

pub struct DirSource {
    pub(super) dir: OsString,
    pub(super) filter: Vec<TFilter>,
    pub(super) action_if_error: Option<TAction>,
}

impl DirSource {
    pub fn build_libraries(&self) -> Result<(Vec<FileSystemLibrary>, Vec<Box<dyn Error>>), std::io::Error> {
        let dir = match std::fs::read_dir(self.dir.clone()) {
            Ok(dir) => dir,
            Err(e) => {
                return Err(e);
            }
        };

        let action = &self.action_if_error;

        Ok(dir.into_iter()
            .filter_map(|path| match path {
                Ok(path) => Some(path.path()),
                Err(e) => {
                    if let Some(action) = action {
                        action(&(Box::new(e) as Box<dyn Error>));
                    }

                    None
                }
            })
            .filter(|path| path.is_file())
            .filter(|path| {
                (&self).filter.iter().all(|f| (*f)(path))
            })
            .partition_map(|path| {
                match FileSource::new(path.into_os_string()).build_library() {
                    Ok(lib) => LR::L(lib),
                    Err(e) => {
                        if let Some(action) = action {
                            action(&e);
                        }

                        LR::R(e)
                    }
                }
            })
        )
    }
}

pub struct DirSourceBuilder {
    pub(super) source: FileSystemSource,
    pub(super) dir: OsString,
    pub(super) filter: Vec<TFilter>,
    pub(super) action_if_error: Option<TAction>,
}

impl DirSourceBuilder {
    pub fn new(source: FileSystemSource, dir: OsString) -> Self {
        Self {
            dir,
            source,
            filter: vec![],
            action_if_error: None,
        }
    }

    pub fn filter_by(mut self, action: fn(&PathBuf) -> bool) -> Self {
        self.filter.push(Box::new(action));

        self
    }

    pub fn if_error_read(mut self, action: fn(&Box<dyn Error>)) -> Self {
        self.action_if_error = Some(Box::new(action));

        self
    }

    pub fn build(mut self) -> FileSystemSource {
        let source = DirSource {
            dir: self.dir,
            filter: self.filter,
            action_if_error: self.action_if_error,
        };

        self.source.source.push(FileSystemSourceEnum::Dir(source));

        self.source
    }

    pub fn from_dir(self, dir: OsString) -> DirSourceBuilder {
        self
            .build()
            .from_dir(dir)
    }

    pub fn from_file(self, path: OsString) -> FileSourceBuilder {
        self
            .build()
            .from_file(path)
    }
}