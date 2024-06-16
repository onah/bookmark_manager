use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::Write;

const SETTINGS_FILE: &str = "bookmarks.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    id: u32,
    url: String,
    reference: u32, // count of how many times the bookmark has been accessed
}

impl Bookmark {
    pub fn new(id: u32, url: String) -> Self {
        let reference = 0;
        Bookmark { id, url, reference }
    }
}

impl Display for Bookmark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {} | {}", self.id, self.url, self.reference)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmarks {
    total_number: u32,
    bookmarks: Vec<Bookmark>,
}

impl Bookmarks {
    pub fn new() -> Self {
        let toml_str = std::fs::read_to_string(SETTINGS_FILE);
        match toml_str {
            Ok(str) => {
                let bookmarks = toml::from_str(&str).unwrap();
                return bookmarks;
            }
            Err(_) => {
                let bookmarks = Bookmarks {
                    total_number: 0,
                    bookmarks: vec![],
                };
                return bookmarks;
            }
        }
    }

    pub fn push(&mut self, url: String) {
        self.total_number += 1;

        let bookmark = Bookmark::new(self.total_number, url);
        self.bookmarks.push(bookmark);
    }

    pub fn countup(&mut self, id: u32) {
        for bookmark in &mut self.bookmarks {
            if bookmark.id == id {
                bookmark.reference += 1;
            }
        }
    }

    pub fn search(&self, id: u32) -> Option<&str> {
        for bookmark in &self.bookmarks {
            if bookmark.id == id {
                return Some(&bookmark.url);
            }
        }
        None
    }

    pub fn save(&self) {
        let toml_str = toml::to_string(&self).unwrap();
        let mut file = std::fs::File::create(SETTINGS_FILE).unwrap();
        file.write_all(toml_str.as_bytes()).unwrap();
    }
}

impl Display for Bookmarks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bookmark in &self.bookmarks {
            write!(f, "{}\n", bookmark)?;
        }
        Ok(())
    }
}
