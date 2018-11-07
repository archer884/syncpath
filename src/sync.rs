mod dir;
mod marker;
mod paths;
mod writer;

use self::{marker::Marker, writer::Writer};
use crate::options::Opt;
use std::collections::HashSet;
use std::io;

pub fn one_way(options: &Opt) -> io::Result<()> {
    let left_tree: HashSet<_> = dir::get_items(&options.left).collect();
    let right_tree: HashSet<_> = dir::get_items(&options.right).collect();

    let mut writer = Writer::from_options(options);

    for file in left_tree.difference(&right_tree) {
        let copy_from = options.left.join(file.path());
        let copy_to = options.right.join(file.path());
        writer.write(&copy_from, &copy_to)?;
    }

    Ok(())
}

pub fn two_way(options: &Opt) -> io::Result<()> {
    let left_tree: HashSet<_> = dir::get_items(&options.left).map(Marker::Left).collect();
    let right_tree: HashSet<_> = dir::get_items(&options.right).map(Marker::Right).collect();

    let mut writer = Writer::from_options(options);

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

        writer.write(&copy_from, &copy_to)?;
    }

    Ok(())
}
