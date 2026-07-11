use clap::Parser;

use crate::getter;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    pub path: String,
}

impl Cli {
    getter!(path, String);
}
