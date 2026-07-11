use clap::Parser;

use crate::getter;

#[derive(Parser)]
pub struct Cli {
    path: String,
}

impl Cli {
    getter!(path, String);
}
