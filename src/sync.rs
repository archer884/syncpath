use crate::entry::{Entry, FileInfo};
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn one_way(left: impl AsRef<Path>, right: impl AsRef<Path>) -> io::Result<()> {
    let left = left.as_ref();
    let right = right.as_ref();

    let left_tree: HashSet<_> = get_items(left).collect();
    let right_tree: HashSet<_> = get_items(right).collect();
    let mut path_creator = PathCreator::new();

    for file in left_tree.difference(&right_tree) {
        let copy_from = left.join(file.path());
        let copy_to = right.join(file.path());

        if let Some(parent) = copy_to.parent() {
            if !parent.as_os_str().is_empty() {
                path_creator.ensure_path(parent)?;
            }
        }

        println!("{}\n -> {}", copy_from.display(), copy_to.display());
        let _ = fs::copy(copy_from, copy_to)?;
    }

    Ok(())
}

pub fn two_way(_left: impl AsRef<Path>, _right: impl AsRef<Path>) -> io::Result<()> {
    unimplemented!()
}

fn get_items<'a>(path: impl AsRef<Path> + 'a) -> impl Iterator<Item = FileInfo> + 'a {
    WalkDir::new(path.as_ref())
        .into_iter()
        .filter_map(move |entry| match entry {
            Err(_) => None,
            Ok(entry) => {
                Entry::from_entry(entry)
                    .ok()
                    .and_then(|entry| match entry.into_file_info() {
                        None => None,
                        Some(mut entry) => {
                            let _ = entry.strip_prefix(path.as_ref());
                            Some(entry)
                        }
                    })
            }
        })
}

#[derive(Debug, Default)]
struct PathCreator {
    set: HashSet<PathBuf>,
}

impl PathCreator {
    fn new() -> PathCreator {
        Default::default()
    }

    fn ensure_path(&mut self, path: impl AsRef<Path> + Into<PathBuf>) -> io::Result<()> {
        if !self.set.contains(path.as_ref()) {
            if !path.as_ref().exists() {
                fs::create_dir_all(path.as_ref())?;
            }
            self.set.insert(path.into());
        }
        Ok(())
    }
}
