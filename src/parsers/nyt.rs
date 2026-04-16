//! New York Times Scraper Module
//!
//! This module provides the [`NytParser`] struct, which is designed to crawl
//! New York Times index pages and extract article URLs, headlines, and body content.
//!
//! It utilizes `reqwest` for networking and `scraper` for HTML parsing.

use chrono::Utc;
use scraper::{Html, Selector};
use std::sync::LazyLock;

/// Global CSS selector for NYT article headlines. It targets the only `h1` element on the page, which is typically the headline.
static NYT_HEADLINE_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("h1").expect("Static Headline CSS selector is malformed"));

/// Global CSS selector for NYT article body paragraphs. It targets all `p` elements, which are commonly used for article content.
static NYT_BODY_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("p").expect("Static Body CSS selector is malformed"));

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
    async fn parse_headline(&self, article_src: &str) -> Option<String> {
        let document = Html::parse_document(article_src);

        document
            .select(&NYT_HEADLINE_SELECTOR)
            .next()
            .map(|element| {
                element
                    .text()
                    .collect::<Vec<_>>()
                    .concat()
                    .trim()
                    .to_string()
            })
    }

    /// Internal helper to fetch and parse the body content from a specific URL.
    async fn parse_body(&self, article_src: &str) -> Option<String> {
        let document = Html::parse_document(article_src);

        let body_texts: Vec<String> = document
            .select(&NYT_BODY_SELECTOR)
            .map(|element| {
                element
                    .text()
                    .collect::<Vec<_>>()
                    .concat()
                    .trim()
                    .to_string()
            })
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

        println!(
            "Found {} article URLs. Fetching and parsing articles...",
            urls.len()
        );
        let mut cnt: i32 = 0;

        for url in &urls {
            cnt += 1;
            println!("Processing URL ({} of {}): {}", cnt, urls.len(), url);
            // if successfully obtained article_src html, then continue, else skip to next url
            let article_src = match reqwest::get(url).await {
                Ok(response) => match response.text().await {
                    Ok(text) => text,
                    Err(_) => continue,
                },
                Err(_) => continue,
            };

            if let (Some(headline), Some(body)) = (
                self.parse_headline(article_src.as_str()).await,
                self.parse_body(article_src.as_str()).await,
            ) {
                articles.push((headline, body));
            }
        }

        articles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_news_urls() {
        let today = Utc::now().format("%Y/%m/%d").to_string();
        let html = format!(
            r#"
            <a href="https://www.nytimes.com/{}/article1.html">Article 1</a>
            <a href="https://www.nytimes.com/{}/article2.html">Article 2</a>
        "#,
            today, today
        );
        let parser = NytParser::new(&html);
        let urls = parser.parse_news_urls();
        assert_eq!(urls.len(), 2);
        assert_eq!(
            urls[0],
            format!("https://www.nytimes.com/{}/article1.html", today)
        );
        assert_eq!(
            urls[1],
            format!("https://www.nytimes.com/{}/article2.html", today)
        );
    }

    #[tokio::test]
    async fn test_parse_headline_and_body() {
        let article_html = r#"
            <html>
                <body>
                    <h1>Test Headline</h1>
                    <p>Paragraph 1.</p>
                    <p>Paragraph 2.</p>
                </body>
            </html>
        "#;
        let parser = NytParser::new("");
        let headline = parser.parse_headline(article_html).await;
        let body = parser.parse_body(article_html).await;

        assert_eq!(headline, Some("Test Headline".to_string()));
        assert_eq!(body, Some("Paragraph 1.\n\nParagraph 2.".to_string()));
    }
}
