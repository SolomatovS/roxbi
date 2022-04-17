use std::ffi::OsStr;
use std::error::Error;


pub trait ILibrary {
    fn id(&self) -> &OsStr;
}

impl PartialEq for dyn ILibrary + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for dyn ILibrary + '_ {}

impl std::fmt::Display for dyn ILibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.id())
    }
}

pub trait ILibrarySource {
    fn generate(&self) -> (Vec<Box<dyn ILibrary>>, Vec<Box<dyn Error>>) ;
}

/*pub struct RepositoryLibrary {
    sources: Vec<Box<dyn ILibrarySource>>,
    libs: Vec<Box<dyn ILibrary>>,
}

impl RepositoryLibrary {
    pub fn new() -> Self {
        Self { sources: vec![], libs: vec![] }
    }

    pub fn add_source(mut self, source: Box<dyn ILibrarySource>) -> Self {
        self.sources.push(source);

        self
    }

    pub fn build_missing_libs(&mut self) {
        let new_libs: Vec<Box<dyn ILibrary>> = self.sources.iter()
            .flat_map(|x| x.generate()).into_iter().collect();

        new_libs.into_iter().for_each(|x|
            if !self.libs.iter().any(|l| l.id() == x.id()) {
                self.libs.push(x);
            }
        );
    }
}
*/