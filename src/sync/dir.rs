use crate::entry::{Entry, FileInfo};
use std::path::Path;
use walkdir::WalkDir;

pub fn get_items<'a>(
    path: impl AsRef<Path> + 'a,
    include_hidden: bool,
) -> impl Iterator<Item = FileInfo> + 'a {
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
        .filter(move |x| include_hidden || !x.is_hidden())
}
