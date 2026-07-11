use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    path: PathBuf,
}

impl Cli {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
