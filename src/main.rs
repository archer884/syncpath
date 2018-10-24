#![feature(dbg_macro)]

#[macro_use]
extern crate structopt;

mod entry;
mod options;
mod sync;

use std::io;

fn main() -> io::Result<()> {
    options::Opt::from_args().execute()
}
