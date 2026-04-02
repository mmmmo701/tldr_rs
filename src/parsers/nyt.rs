//! New York Times Scraper Module
//!
//! This module provides the [`NytParser`] struct, which is designed to crawl
//! New York Times index pages and extract article URLs, headlines, and body content.
//!
//! It utilizes `reqwest` for networking and `scraper` for HTML parsing.


use scraper::{Html, Selector};
use std::sync::LazyLock;
use chrono::Utc;

/// Global CSS selector for NYT article headlines. It targets the only `h1` element on the page, which is typically the headline.
static NYT_HEADLINE_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("h1").expect("Static Headline CSS selector is malformed")
});

/// Global CSS selector for NYT article body paragraphs. It targets all `p` elements, which are commonly used for article content.
static NYT_BODY_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("p").expect("Static Body CSS selector is malformed")
});

/// A parser responsible for extracting news data from New York Times HTML source code.
pub struct NytParser<'a> {
    /// The raw HTML source code of the index page.
    src: &'a str,
}

impl<'a> NytParser<'a> {
    /// Creates a new instance of `NytParser` with the provided HTML source.
    ///
    /// # Arguments
    /// * `src` - A string slice containing the HTML of a New York Times landing page.
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }

    /// Parses the initial HTML source to find all article links.
    /// This looks for anchor tags and filters them based on the current date in the URL pattern.
    pub fn parse_news_urls(&self) -> Vec<String> {
        let date = Utc::now().format("%Y/%m/%d").to_string();
        let mut urls = Vec::new();
        let document = Html::parse_document(self.src);
        let selector = Selector::parse("a").expect("Failed to parse selector");

        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                let start: String = format!("https://www.nytimes.com/{}", date);
                if href.starts_with(&start) && href.ends_with(".html") {
                    urls.push(href.to_string());
                }
            }
        }

        urls
    }

    /// Internal helper to fetch and parse a headline from a specific URL.
    async fn parse_headline(&self, url: &str) -> Option<String> {
        let article_src = reqwest::get(url).await.ok()?.text().await.ok()?;
        let document = Html::parse_document(&article_src);

        document.select(&NYT_HEADLINE_SELECTOR).next().map(|element| {
            element
                .text()
                .collect::<Vec<_>>()
                .concat()
                .trim()
                .to_string()
        })
    }

    /// Internal helper to fetch and parse the body content from a specific URL.
    async fn parse_body(&self, url: &str) -> Option<String> {
        let article_src = reqwest::get(url).await.ok()?.text().await.ok()?;
        let document = Html::parse_document(&article_src);

        let body_texts: Vec<String> = document
            .select(&NYT_BODY_SELECTOR)
            .map(|element| element.text().collect::<Vec<_>>().concat().trim().to_string())
            .collect();

        if body_texts.is_empty() {
            None
        } else {
            Some(body_texts.join("\n\n"))
        }
    }

    /// Parses the top articles from the initial HTML source.
    /// This method first extracts article URLs, then fetches each article to parse its headline and body. It returns a vector of tuples, where each tuple contains the headline and body of an article
    pub async fn parse_top_articles(&self) -> Vec<(String, String)> {
        let urls = self.parse_news_urls();
        let mut articles = Vec::new();

        for url in urls {
            if let (Some(headline), Some(body)) = (self.parse_headline(&url).await, self.parse_body(&url).await) {
                articles.push((headline, body));
            }
        }

        articles
    }
}