use crate::bookmark::BookmarksAccessor;
use crate::fileio::FileSystemAccessor;
use crate::settings;
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
    pub fn run(&self) {
        match &self.subcommand {
            SubCommand::Add { url } => {
                add(url);
            }
            SubCommand::List => {
                list();
            }
            SubCommand::Execute { id } => {
                execute(*id);
            }
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
