use crate::bookmark::BookmarksAccessor;
use crate::fileio::FileSystemAccessor;
use crate::process::{ProcessExecutor, ProcessExecutorImpl};
use crate::settings;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Add { url: String },
    List,
    Execute { id: u32 },
}

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match &self.subcommand {
            SubCommand::Add { url } => {
                add(url)?;
            }
            SubCommand::List => {
                list()?;
            }
            SubCommand::Execute { id } => {
                let executor = ProcessExecutorImpl;
                execute(*id, executor)?;
            }
        }
        Ok(())
    }
}

fn add(url: &str) -> Result<()> {
    let settings = settings::Settings::new(FileSystemAccessor)?;
    let bookmark_accessor = BookmarksAccessor::new(settings.get_bookmark_file());
    let mut bookmarks = bookmark_accessor.load(FileSystemAccessor)?;

    bookmarks.push(url.to_string());
    println!("Bookmarks: {:?}", bookmarks);
    let accessor = FileSystemAccessor;
    bookmark_accessor.save(accessor, &bookmarks)?;

    Ok(())
}

fn list() -> Result<()> {
    let settings = settings::Settings::new(FileSystemAccessor)?;
    let bookmark_accessor = BookmarksAccessor::new(settings.get_bookmark_file());
    let bookmarks = bookmark_accessor.load(FileSystemAccessor)?;

    println!("{}", bookmarks);
    Ok(())
}

fn execute<T: ProcessExecutor>(id: u32, executor: T) -> Result<()> {
    let settings = settings::Settings::new(FileSystemAccessor)?;
    let bookmark_accessor = BookmarksAccessor::new(settings.get_bookmark_file());
    let mut bookmarks = bookmark_accessor.load(FileSystemAccessor)?;

    let url = bookmarks.search(id).context("Bookmark not found")?;
    let browser = settings.get_browser();
    executor.execute(browser, url)?;

    bookmarks.countup(id);
    let accessor = FileSystemAccessor;
    bookmark_accessor.save(accessor, &bookmarks)?;

    Ok(())
}
