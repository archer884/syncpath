use crate::sync;
use std::io;
use std::path::PathBuf;

/// Used to set program options.
///
/// For one-way operations, the `left` path represents the "from" path and the `right` path
/// represents the "to" path. For two-way operations, they are basically interchangeable.
///
/// The "sync-both" option represents a two-way operation.
///
/// By default, syncpath won't actually do anything; it'll just tell you what it *would* do
/// if you asked it to do something. In order to ask syncpath to do something, set the "write"
/// flag.
///
/// Again by default, syncpath writes what you want it to do; if you instead ask it to make
/// changes to your file system, it'll skip writing a summary of these changes to the console.
/// If you'd still like to see this summary, set the "verbose" flag.
///
/// And lastly, by default syncpath will ignore all hidden files. If you'd like to include them,
/// set the all_files flag.
#[derive(StructOpt)]
#[structopt(name = "syncpath", about = "syncs directories")]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    pub left: PathBuf,
    #[structopt(parse(from_os_str))]
    pub right: PathBuf,
    #[structopt(short = "x", long = "two-way")]
    pub sync_both: bool,
    #[structopt(short = "w", long = "write")]
    pub write: bool,
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
    #[structopt(short = "a", long = "all-files")]
    pub all_files: bool,
}

impl Opt {
    pub fn from_args() -> Opt {
        structopt::StructOpt::from_args()
    }

    pub fn execute(self) -> io::Result<()> {
        if self.sync_both {
            sync::two_way(&self)
        } else {
            sync::one_way(&self)
        }
    }
}
