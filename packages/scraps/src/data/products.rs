use scraper::{Html, Selector};
pub fn parse(selector: &str, document: Html) -> Selector {
  Selector::parse(selector).unwrap()
}
