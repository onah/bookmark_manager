mod bookmark;
mod cli;
mod fileio;
mod settings;

use bookmark::BookmarksAccessor;
use clap::Parser;
use cli::{Cli, SubCommand};
use core::str;
use fileio::FileSystemAccessor;

fn main() {
    let args = Cli::parse();
    match args.get_subcommand() {
        SubCommand::Add { url } => {
            add(&url);
        }
        SubCommand::List => {
            list();
        }
        SubCommand::Execute { id } => {
            execute(*id);
        }
    }
}

fn add(url: &str) {
    let settings = settings::Settings::new(FileSystemAccessor);
    let bookmark_accessor = BookmarksAccessor::new(settings.get_bookmark_file());
    let mut bookmarks = bookmark_accessor.load(FileSystemAccessor);

    bookmarks.push(url.to_string());
    println!("Bookmarks: {:?}", bookmarks);
    let accessor = FileSystemAccessor;
    bookmark_accessor.save(accessor, &bookmarks);
}

fn list() {
    let settings = settings::Settings::new(FileSystemAccessor);
    let bookmark_accessor = BookmarksAccessor::new(settings.get_bookmark_file());
    let bookmarks = bookmark_accessor.load(FileSystemAccessor);

    println!("{}", bookmarks);
}

fn execute(id: u32) {
    let settings = settings::Settings::new(FileSystemAccessor);
    let bookmark_accessor = BookmarksAccessor::new(settings.get_bookmark_file());
    let mut bookmarks = bookmark_accessor.load(FileSystemAccessor);

    let url = bookmarks.search(id).unwrap();
    //println!("Executing: {}", url);
    let browswer = r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe";
    let _output = std::process::Command::new(browswer)
        .arg(url)
        .output()
        .expect("failed to execute process");

    bookmarks.countup(id);
    let accessor = FileSystemAccessor;
    bookmark_accessor.save(accessor, &bookmarks);
}
