use std::{fs::File, process::ExitCode};

use clap::Parser;
use zinspect::{app::App, cli::Cli, match_error};
use zip::ZipArchive;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let file = match_error!(File::open(cli.path()));
    let zip = match_error!(ZipArchive::new(file));

    let app = App::new(zip);

    ExitCode::SUCCESS
}
