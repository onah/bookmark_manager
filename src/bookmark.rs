use crate::fileio::FileAccessor;
use crate::settings::{self, Settings};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::Write;

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
    pub fn new<T: FileAccessor>(file_reader: T) -> Self {
        let settings = Settings::new();

        let toml_str = file_reader.read_to_string(&settings.get_bookmark_file());
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
        let settings = settings::Settings::new();

        let toml_str = toml::to_string(&self).unwrap();
        println!("{}", settings.get_bookmark_file());
        let mut file = std::fs::File::create(settings.get_bookmark_file()).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bookmark() {
        let bookmark = Bookmark::new(1, "https://example.com".to_string());
        assert_eq!(bookmark.id, 1);
        assert_eq!(bookmark.url, "https://example.com");
        assert_eq!(bookmark.reference, 0);
    }

    pub struct MockFileReader;
    impl FileAccessor for MockFileReader {
        fn read_to_string(&self, _path: &str) -> Result<String, std::io::Error> {
            Ok("total_number = 1\n[[bookmarks]]\nid = 1\nurl = \"https://example.com\"\nreference = 0\n".to_string())
        }
    }

    #[test]
    fn test_bookmarks_new() {
        let reader = MockFileReader;
        let bookmarks = Bookmarks::new(reader);
        assert_eq!(bookmarks.total_number, 1);
    }

    #[test]
    fn test_bookmarks_push() {
        let reader = MockFileReader;
        let mut bookmarks = Bookmarks::new(reader);
        bookmarks.push("https://example.com".to_string());
        assert_eq!(bookmarks.total_number, 2);
        assert_eq!(bookmarks.bookmarks.len(), 2);
    }

    #[test]
    fn test_bookmarks_countup() {
        let reader = MockFileReader;
        let mut bookmarks = Bookmarks::new(reader);
        bookmarks.countup(1);
        assert_eq!(bookmarks.bookmarks[0].reference, 1);
    }

    #[test]
    fn test_bookmarks_search() {
        let reader = MockFileReader;
        let bookmarks = Bookmarks::new(reader);
        let url = bookmarks.search(1).unwrap();
        assert_eq!(url, "https://example.com");
    }
}
