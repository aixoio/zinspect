use clap::Parser;
use zinspect::cli::Cli;

fn main() {
    let cli = Cli::parse();

    println!("file: {:?}", cli.path());
}
