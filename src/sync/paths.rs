use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Debug, Default)]
pub struct PathCreator {
    set: HashSet<PathBuf>,
}

impl PathCreator {
    pub fn new() -> PathCreator {
        Default::default()
    }

    pub fn ensure_path(&mut self, path: impl AsRef<Path> + Into<PathBuf>) -> io::Result<()> {
        if !self.set.contains(path.as_ref()) {
            if !path.as_ref().exists() {
                fs::create_dir_all(path.as_ref())?;
            }
            self.set.insert(path.into());
        }
        Ok(())
    }
}
