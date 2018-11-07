mod dir;
mod marker;
mod paths;

use self::{marker::Marker, paths::PathCreator};
use crate::options::Opt;
use std::collections::HashSet;
use std::{fs, io};

pub fn one_way(options: &Opt) -> io::Result<()> {
    let left_tree: HashSet<_> = dir::get_items(&options.left).collect();
    let right_tree: HashSet<_> = dir::get_items(&options.right).collect();
    let mut path_creator = PathCreator::new();

    let write_results = !options.write || options.verbose;

    for file in left_tree.difference(&right_tree) {
        let copy_from = options.left.join(file.path());
        let copy_to = options.right.join(file.path());

        if write_results {
            println!("{}\n -> {}", copy_from.display(), copy_to.display());
        }

        if options.write {
            if let Some(parent) = copy_to.parent() {
                if !parent.as_os_str().is_empty() {
                    path_creator.ensure_path(parent)?;
                }
            }
            
            let _ = fs::copy(copy_from, copy_to)?;
        }
    }

    Ok(())
}

pub fn two_way(options: &Opt) -> io::Result<()> {
    let left_tree: HashSet<_> = dir::get_items(&options.left).map(Marker::Left).collect();
    let right_tree: HashSet<_> = dir::get_items(&options.right).map(Marker::Right).collect();
    let mut path_creator = PathCreator::new();

    let write_results = !options.write || options.verbose;

    for file in left_tree.symmetric_difference(&right_tree) {
        let (copy_from, copy_to) = match file {
            Marker::Left(file) => (
                options.left.join(file.path()),
                options.right.join(file.path()),
            ),
            Marker::Right(file) => (
                options.right.join(file.path()),
                options.left.join(file.path()),
            ),
        };

        if write_results {
            println!("{}\n -> {}", copy_from.display(), copy_to.display());
        }

        if options.write {
            if let Some(parent) = copy_to.parent() {
                if !parent.as_os_str().is_empty() {
                    path_creator.ensure_path(parent)?;
                }
            }

            let _ = fs::copy(copy_from, copy_to)?;
        }
    }

    Ok(())
}
