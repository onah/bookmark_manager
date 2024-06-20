mod bookmark;
mod cli;
mod fileio;
mod settings;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();
    args.run();
}
