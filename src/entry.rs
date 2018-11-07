use std::hash::{Hash, Hasher};
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf, StripPrefixError};
use std::time::SystemTime as Time;
use walkdir::DirEntry;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Entry {
    Directory(DirectoryInfo),
    File(FileInfo),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct DirectoryInfo {
    path: PathBuf,
    created: Option<Time>,
    modified: Option<Time>,
}

#[derive(Debug, Eq)]
pub struct FileInfo {
    path: PathBuf,
    len: u64,
    created: Option<Time>,
    modified: Option<Time>,
}

impl FileInfo {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn strip_prefix(&mut self, prefix: impl AsRef<Path>) -> Result<(), StripPrefixError> {
        self.path = self.path.strip_prefix(prefix)?.to_owned();
        Ok(())
    }
}

// Attempt to fix screwy bullshit...
impl Hash for FileInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len.hash(state);
        self.path.hash(state);
    }
}

impl PartialEq for FileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.path == other.path
    }
}

impl Entry {
    pub fn from_entry(entry: DirEntry) -> io::Result<Entry> {
        let meta = entry.metadata()?;
        if meta.is_file() {
            Ok(Entry::File(FileInfo {
                path: entry.into_path(),
                len: meta.len(),
                created: meta.created().ok(),
                modified: meta.modified().ok(),
            }))
        } else if meta.is_dir() {
            Ok(Entry::Directory(DirectoryInfo {
                path: entry.into_path(),
                created: meta.created().ok(),
                modified: meta.modified().ok(),
            }))
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "Entry must be a file or directory",
            ))
        }
    }

    pub fn into_file_info(self) -> Option<FileInfo> {
        if let Entry::File(info) = self {
            Some(info)
        } else {
            None
        }
    }
}
