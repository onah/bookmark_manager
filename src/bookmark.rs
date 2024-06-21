use crate::fileio::FileAccessor;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
        Bookmarks {
            total_number: 0,
            bookmarks: vec![],
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
}

impl Display for Bookmarks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bookmark in &self.bookmarks {
            write!(f, "{}\n", bookmark)?;
        }
        Ok(())
    }
}

pub struct BookmarksAccessor {
    bookmarks_path: String,
}

impl BookmarksAccessor {
    pub fn new(path: &str) -> Self {
        BookmarksAccessor {
            bookmarks_path: path.to_string(),
        }
    }

    pub fn load<T: FileAccessor>(&self, file_accessor: T) -> Result<Bookmarks> {
        let toml_str = file_accessor.read_to_string(&self.bookmarks_path);
        match toml_str {
            Ok(str) => {
                let bookmarks = toml::from_str(&str)?;
                return Ok(bookmarks);
            }
            Err(_) => {
                let bookmarks = Bookmarks::new();
                return Ok(bookmarks);
            }
        }
    }

    pub fn save<T: FileAccessor>(&self, file_accessor: T, bookmarks: &Bookmarks) -> Result<()> {
        let toml_str = toml::to_string(&bookmarks)?;
        file_accessor.write_all(&self.bookmarks_path, &toml_str)?;

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

    struct MockFileReader;
    impl FileAccessor for MockFileReader {
        fn read_to_string(&self, _path: &str) -> Result<String, std::io::Error> {
            Ok("total_number = 1\n[[bookmarks]]\nid = 1\nurl = \"https://example.com\"\nreference = 0\n".to_string())
        }
        fn write_all(&self, _path: &str, _content: &str) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    #[test]
    fn test_bookmarks_new() {
        let bookmarks = Bookmarks::new();
        assert_eq!(bookmarks.total_number, 0);
    }

    #[test]
    fn test_bookmarks_push() {
        let mut bookmarks = Bookmarks::new();
        bookmarks.push("https://example.com".to_string());
        assert_eq!(bookmarks.total_number, 1);
        assert_eq!(bookmarks.bookmarks.len(), 1);
    }

    #[test]
    fn test_bookmarks_countup() {
        let mut bookmarks = Bookmarks::new();
        bookmarks.push("https://example.com".to_string());
        bookmarks.countup(1);
        assert_eq!(bookmarks.bookmarks[0].reference, 1);
    }

    #[test]
    fn test_bookmarks_search_none() {
        let bookmarks = Bookmarks::new();
        assert_eq!(bookmarks.search(1), None);
    }

    #[test]
    fn test_bookmarks_search_some() {
        let mut bookmarks = Bookmarks::new();
        bookmarks.push("https://example.com".to_string());
        assert_eq!(bookmarks.search(1), Some("https://example.com"));
    }

    #[test]
    fn test_bookmarks_accessor_load() {
        let accessor = BookmarksAccessor::new("bookmarks.toml");
        let bookmarks = accessor.load(MockFileReader).unwrap();
        assert_eq!(bookmarks.total_number, 1);
        assert_eq!(bookmarks.bookmarks.len(), 1);
    }

    #[test]
    fn test_bookmarks_accessor_save() {
        let accessor = BookmarksAccessor::new("bookmarks.toml");
        let mut bookmarks = Bookmarks::new();
        bookmarks.push("https://example.com".to_string());
        accessor.save(MockFileReader, &bookmarks).unwrap();
    }
}
