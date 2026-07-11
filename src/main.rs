use std::{fs::File, process::ExitCode};

use clap::Parser;
use zinspect::{cli::Cli, match_error};
use zip::ZipArchive;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let file = match_error!(File::open(cli.path()));
    let zip = match_error!(ZipArchive::new(file));

    for file in zip.file_names() {
        println!("zip: {}", file);
    }

    ExitCode::SUCCESS
}
