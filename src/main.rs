mod bookmark;
mod cli;

use bookmark::Bookmarks;
use clap::Parser;
use cli::{Cli, SubCommand};
use core::str;

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
    let mut bookmarks = Bookmarks::new();

    bookmarks.push(url.to_string());
    println!("Bookmarks: {:?}", bookmarks);
    bookmarks.save();
}

fn list() {
    let bookmarks = Bookmarks::new();
    println!("{}", bookmarks);
}

fn execute(id: u32) {
    let mut bookmarks = Bookmarks::new();

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
