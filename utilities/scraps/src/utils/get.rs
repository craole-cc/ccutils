use anyhow::{Context, Result};
use logline::debug;
use reqwest::get;
use scraper::{Html, Selector};
// use tracing::debug;

/// Fetches the content from the given URL and returns it as a `String`.
///
/// # Arguments
///
/// * `url` - A `&str` containing the URL to fetch the content from.
///
/// # Returns
///
/// * `Result<String>` - On success, returns the HTML content as a `String`. On failure, returns an error.
pub async fn html_content(url: &str) -> Result<String> {
    let response = get(url)
        .await
        .context(format!("Failed to send URL request for: '{}'", url))?;
    debug!("{:#?}", response);

    let content = response
        .text()
        .await
        .context(format!("Failed to read response text from: '{}'", url))?;
    debug!("{:#?}", content);

    Ok(content)
}

/// Parses the given HTML content string and returns a `Html` document.
///
/// # Arguments
///
/// * `content` - A `String` containing the HTML content to be parsed.
///
/// # Returns
///
/// * `Result<Html>` - On success, returns a `Html` document. On failure, returns an error.
pub fn html_document(content: String) -> Result<Html> {
    let html = Html::parse_document(&content);
    debug!("{:#?}", html);
    Ok(html)
}
