use super::paths::PathCreator;
use crate::options::Opt;
use std::path::Path;
use std::{fs, io};

pub struct Writer {
    path_creator: PathCreator,
    verbose: bool,
    write_files: bool,
}

impl Writer {
    pub fn from_options(opt: &Opt) -> Writer {
        Writer {
            path_creator: PathCreator::new(),
            verbose: !opt.write || opt.verbose,
            write_files: opt.write,
        }
    }

    pub fn write(&mut self, copy_from: &Path, copy_to: &Path) -> io::Result<()> {
        if self.verbose {
            println!("{}\n -> {}", copy_from.display(), copy_to.display());
        }

        if self.write_files {
            if let Some(parent) = copy_to.parent() {
                if !parent.as_os_str().is_empty() {
                    self.path_creator.ensure_path(parent)?;
                }
            }

            let _ = fs::copy(copy_from, copy_to)?;
        }

        Ok(())
    }
}
