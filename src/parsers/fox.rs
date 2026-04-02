//! Fox News Scraper Module
//!
//! This module provides the [`FoxParser`] struct, which is designed to crawl
//! Fox News index pages and extract article URLs, headlines, and body content.
//!
//! It utilizes `reqwest` for networking and `scraper` for HTML parsing.

use reqwest;
use scraper::{Html, Selector};
use std::sync::LazyLock;

/// Global CSS selector for Fox article headlines.
static FOX_HEADLINE_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("div.article-meta-upper h1.headline")
        .expect("Static Headline CSS selector is malformed")
});

/// Global CSS selector for Fox article body paragraphs.
static FOX_BODY_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("div.article-body p")
        .expect("Static Body CSS selector is malformed")
});

/// A parser responsible for extracting news data from Fox News HTML source code.
///
/// The parser holds a reference to the HTML source string and provides methods
/// to extract links and full article content asynchronously.
pub struct FoxParser<'a> {
    /// The raw HTML source code of the index page.
    src: &'a str,
}

impl<'a> FoxParser<'a> {
    /// Creates a new instance of `FoxParser` with the provided HTML source.
    ///
    /// # Arguments
    /// * `src` - A string slice containing the HTML of a Fox News landing page.
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }

    /// Parses the initial HTML source to find all article links.
    ///
    /// This looks for `article` elements with a class containing 'story-' 
    /// and extracts the `href` attribute from the nested anchor tag.
    pub fn parse_news_urls(&self) -> Vec<String> {
        let mut urls = Vec::new();
        let document = Html::parse_document(self.src);
        let selector = Selector::parse("article[class*='story-'] div.m > a")
            .expect("Failed to parse selector");

        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                urls.push(href.to_string());
            }
        }

        urls
    }

    /// Internal helper to fetch and parse a headline from a specific URL.
    async fn parse_headline(&self, url: &str) -> Option<String> {
        let article_src = reqwest::get(url).await.ok()?.text().await.ok()?;
        let document = Html::parse_document(&article_src);

        document.select(&FOX_HEADLINE_SELECTOR).next().map(|element| {
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
            .select(&FOX_BODY_SELECTOR)
            .map(|element| element.text().collect::<Vec<_>>().concat().trim().to_string())
            .collect();

        if body_texts.is_empty() {
            None
        } else {
            Some(body_texts.join("\n\n"))
        }
    }

    /// Parses the top articles from the initial HTML source.
    /// This method first extracts article URLs, then fetches each article to parse its headline and body.
    /// It returns a vector of tuples, where each tuple contains the headline and body of an article.
    pub async fn parse_top_articles(&self) -> Vec<(String, String)> {
        let urls = self.parse_news_urls();
        let mut articles = Vec::new();

        let urls = urls.into_iter().take(20).collect::<Vec<_>>();

        for url in urls {
            if let Some(headline) = self.parse_headline(&url).await {
                if let Some(body) = self.parse_body(&url).await {
                    articles.push((headline, body));
                }
            }
        }

        articles
    }
}