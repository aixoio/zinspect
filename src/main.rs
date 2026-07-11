use std::{fs::File, process::ExitCode};

use clap::Parser;
use zinspect::{app::App, cli::Cli, handle_error, match_error};
use zip::ZipArchive;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let file = match_error!(File::open(cli.path()));
    let zip = match_error!(ZipArchive::new(file));

    let mut app = match_error!(App::new(zip, cli.path.into_boxed_str()));
    let mut terminal = ratatui::init();

    let result = app.run(&mut terminal);

    ratatui::restore();

    handle_error!(result);
    ExitCode::SUCCESS
}
