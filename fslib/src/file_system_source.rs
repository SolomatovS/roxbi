use std::ffi::OsString;
use std::vec;
use std::error::Error;

use lib::{ILibrary, ILibrarySource};

use super::dir_source::*;
use super::file_source::*;

pub enum FileSystemSourceEnum {
    File(FileSource),
    Dir(DirSource),
}

impl FileSystemSourceEnum {
    pub fn generate(&self) -> (Vec<Box<dyn ILibrary>>, Vec<Box<dyn Error>>) {
        match *self {
            Self::File(ref file) => {
                let file = file.build_library();

                match file {
                    Ok(file) => (vec![Box::new(file)], vec![]),
                    Err(e) => (vec![], vec![e])
                }
            },
            Self::Dir(ref dir) => {
                match dir.build_libraries() {
                    Ok((files, errors)) => {
                        let files = files.into_iter()
                            .map(|f| (Box::new(f) as Box<dyn ILibrary>))
                            .collect();

                        (files, errors)
                    },
                    Err(e) => (vec![], vec![Box::new(e)])
                }
            },
        }
    }
}

pub struct FileSystemSource {
    pub (crate) source: Vec<FileSystemSourceEnum>
}

impl FileSystemSource {
    pub fn new() -> Self {
        Self {
            source: vec![],
        }
    }

    pub fn from_file(self, path: OsString) -> FileSourceBuilder {
        FileSourceBuilder::new(self, path)
    }

    pub fn from_dir(self, dir: OsString) -> DirSourceBuilder {
        DirSourceBuilder::new(self, dir)
    }
}

impl ILibrarySource for FileSystemSource {
    fn generate(&self) -> (Vec<Box<dyn ILibrary>>, Vec<Box<dyn Error>>) {
        let mut files = vec![];
        let mut errors = vec![];

        self.source.iter()
            .for_each(|x| {
                let (mut f, mut e) = x.generate();

                files.append(&mut f);
                errors.append(&mut e);
            });

        (files, errors)
    }
}