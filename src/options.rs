use crate::sync;
use std::io;
use std::path::PathBuf;

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
