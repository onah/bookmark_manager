mod bookmark;
mod cli;
mod fileio;
mod process;
mod settings;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run()?;
    Ok(())
}
