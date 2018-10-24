#[macro_use]
extern crate structopt;

mod entry;
mod options;
mod sync;

use std::io;

// FIXME:
//
// - This stupid tool is way too happy to copy over things like .DS_Store files.
// - Add a dry run mode. (Duh?! How did you get the real thing without a trial version?)
// - Implement two-way mode.
// - Add 'silent' mode.

fn main() -> io::Result<()> {
    options::Opt::from_args().execute()
}
