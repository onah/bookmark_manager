use reqwest::blocking;
use scraper::{Html, Selector};
use std::error::Error;

pub trait GetHtmlTitle {
    fn get_html_title(&self, url: &str) -> String;
}

pub struct GetHtmlTitleImpl;
impl GetHtmlTitle for GetHtmlTitleImpl {
    fn get_html_title(&self, url: &str) -> String {
        match get_html_title_impl(url) {
            Ok(title) => title,
            Err(_) => "".to_string(),
        }
    }
}

fn get_html_title_impl(url: &str) -> Result<String, Box<dyn Error>> {
    let body = blocking::get(url)?.text()?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("title")?;

    if let Some(title) = document.select(&selector).next() {
        Ok(title.inner_html())
    } else {
        Ok("".to_string())
    }
}
