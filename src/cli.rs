use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

impl Cli {
    pub fn get_subcommand(&self) -> &SubCommand {
        &self.subcommand
    }
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Add { url: String },
    List,
    Execute { id: u32 },
}
