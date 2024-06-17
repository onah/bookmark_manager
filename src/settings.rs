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
    pub fn new() -> Self {
        let settings = match std::fs::read_to_string("settings.toml") {
            Ok(str) => toml::from_str(&str).unwrap(),
            Err(_) => Settings::default(),
        };
        settings
    }

    pub fn get_bookmark_file(&self) -> &str {
        &self.bookmarks_file
    }
}
