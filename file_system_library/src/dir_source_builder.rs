use std::ffi::OsString;
use std::path::PathBuf;
use std::rc::Rc;

use super::file_system_source::*;
use super::dir_source::*;

pub struct DirSourceBuilder {
    source: FileSystemSource,
    dir: OsString,
    filter: Option<Rc<dyn Fn(&PathBuf) -> bool>>,
    action_if_error: Option<Rc<dyn Fn(std::io::Error)>>,
}

impl DirSourceBuilder {
    pub fn new(source: FileSystemSource, dir: OsString) -> Self {
        Self {
            dir,
            source,
            filter: None,
            action_if_error: None,
        }
    }

    pub fn filter_by(mut self, action: Rc<dyn Fn(&PathBuf) -> bool>) -> Self {
        self.filter = Some(action);

        self
    }

    pub fn if_error_read(mut self, action: Rc<dyn Fn(std::io::Error)>) -> Self {
        self.action_if_error = Some(action);

        self
    }

    pub fn read_files(mut self) -> FileSystemSource {
        let source = DirSource {
            dir: self.dir,
            filter: self.filter,
            action_if_error: self.action_if_error,
        };

        self.source.dirs.push(source);

        self.source
    }
}
