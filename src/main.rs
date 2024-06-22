mod bookmark;
mod cli;
mod fileio;
mod html;
mod process;
mod settings;

use anyhow;
use clap::Parser;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.run()?;
    Ok(())
}
