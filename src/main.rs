mod bookmark;
mod cli;
mod fileio;
mod settings;

use bookmark::Bookmarks;
use clap::Parser;
use cli::{Cli, SubCommand};
use core::str;
use fileio::FileSystemReader;

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
    let reader = FileSystemReader;
    let mut bookmarks = Bookmarks::new(reader);

    bookmarks.push(url.to_string());
    println!("Bookmarks: {:?}", bookmarks);
    bookmarks.save();
}

fn list() {
    let reader = FileSystemReader;
    let bookmarks = Bookmarks::new(reader);
    println!("{}", bookmarks);
}

fn execute(id: u32) {
    let reader = FileSystemReader;
    let mut bookmarks = Bookmarks::new(reader);

    let url = bookmarks.search(id).unwrap();
    //println!("Executing: {}", url);
    let browswer = r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe";
    let _output = std::process::Command::new(browswer)
        .arg(url)
        .output()
        .expect("failed to execute process");

    bookmarks.countup(id);
    bookmarks.save();
}
