//! News Websites Module
//! This module defines the `NewsWebsite` struct, which represents a news website with a URL and provides a method to fetch the raw HTML content of the main page asynchronously using the `reqwest` library. The `fetch_raw_main_page` method sends an HTTP GET request to the specified URL and returns the response body as a string, or an error if the request fails.

use reqwest;

/// A struct representing a news website with its URL.
#[derive(Debug, Clone)]
pub struct NewsWebsite {
    url: String,
}

impl NewsWebsite {
    /// Creates a new instance of `NewsWebsite` with the provided URL.
    /// # Arguments
    /// * `my_url` - A string slice containing the URL of the news website.
    pub fn from_url(my_url: &str) -> Self {
        Self {
            url: my_url.to_string(),
        }
    }

    /// Asynchronously fetches the raw HTML content of the main page of the news website.
    /// It sends an HTTP GET request to the URL stored in the `NewsWebsite` instance and returns the response body as a string. If the request fails, it returns an error.
    pub async fn fetch_raw_main_page(&self) -> Result<String, Box<dyn std::error::Error>> {
        let body = reqwest::get(&self.url)
            .await?
            .text()
            .await?;
        Ok(body)
    }
}
