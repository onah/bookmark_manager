use crate::fileio::FileAccessor;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default = "default_browser")]
    browser: String,
    #[serde(default = "default_bookmarks_file")]
    bookmarks_file: String,
}

fn default_browser() -> String {
    r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe".to_string()
}

fn default_bookmarks_file() -> String {
    "bookmarks.toml".to_string()
}

impl Default for Settings {
    fn default() -> Self {
        let browser = default_browser();
        let bookmarks_file = default_bookmarks_file();
        Settings {
            browser,
            bookmarks_file,
        }
    }
}

impl Settings {
    pub fn new<T: FileAccessor>(file_accessor: T) -> Result<Self> {
        let settings = match file_accessor.read_to_string("settings.toml") {
            Ok(str) => toml::from_str(&str)?,
            Err(_) => Settings::default(),
        };
        Ok(settings)
    }

    pub fn get_bookmark_file(&self) -> &str {
        &self.bookmarks_file
    }

    pub fn get_browser(&self) -> &str {
        &self.browser
    }
}
