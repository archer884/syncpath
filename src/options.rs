use crate::sync;
use std::io;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(name = "syncpath", about = "syncs directories")]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    left: PathBuf,
    #[structopt(parse(from_os_str))]
    right: PathBuf,
    #[structopt(short = "x", long = "two-way")]
    sync_both: bool,
}

impl Opt {
    pub fn from_args() -> Opt {
        structopt::StructOpt::from_args()
    }

    pub fn execute(self) -> io::Result<()> {
        if self.sync_both {
            sync::two_way(self.left, self.right)
        } else {
            sync::one_way(self.left, self.right)
        }
    }
}
