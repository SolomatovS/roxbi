use std::ffi::OsString;
use std::path::PathBuf;
use std::rc::Rc;
use std::vec;

use lib::ILibrary;

use super::file_source::*;

pub struct DirSource {
    pub(super) dir: OsString,
    pub(super) filter: Option<Rc<dyn Fn(&PathBuf) -> bool>>,
    pub(super) action_if_error: Option<Rc<dyn Fn(std::io::Error)>>,
}

impl DirSource {
    pub fn build_libraries(&self) -> Vec<Box<dyn ILibrary>> {
        let dir = match std::fs::read_dir(self.dir.clone()) {
            Ok(dir) => dir,
            Err(e) => {
                if let Some(a) = &self.action_if_error {
                    a(e);
                }

                return vec![];
            }
        };

        let filter = match &self.filter {
            Some(filter) => filter,
            None => return vec![],
        };

        let action = &self.action_if_error;

        dir.into_iter()
            .filter_map(|path| match path {
                Ok(path) => Some(path.path()),
                Err(e) => {
                    if let Some(action) = action {
                        action(e);
                    }

                    None
                }
            })
            .filter(|path| path.is_file())
            .filter(|p| (*filter)(p))
            .map(|path| FileSource::new(path.into_os_string()).build_library())
            .collect()
    }
}
