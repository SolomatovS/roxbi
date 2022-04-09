use std::vec;
use std::ffi::OsString;

use lib::{ILibrary, ILibrarySource};

use super::file_source::*;
use super::dir_source::*;
use super::dir_source_builder::DirSourceBuilder;

pub struct FileSystemSource {
    pub(super) files: Vec<FileSource>,
    pub(super) dirs: Vec<DirSource>,
}

impl FileSystemSource {
    pub fn new() -> Self {
        Self { files: vec![], dirs: vec![] }
    }

    pub fn from_file(mut self, path: OsString) -> Self {
        let generator = FileSource::new(path);

        self.files.push(generator);

        self
    }

    pub fn from_dir(self, dir: OsString) -> DirSourceBuilder {
        DirSourceBuilder::new(self, dir)
    }
}

impl ILibrarySource for FileSystemSource {
    fn generate(&self) -> Vec<Box<dyn ILibrary>> {
        self.dirs
            .iter()
            .flat_map(|x| x.build_libraries())
            .chain(self.files.iter().map(|x| x.build_library()))
            .collect()
    }
}
