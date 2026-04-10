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
    async fn parse_headline(&self, article_src: &str) -> Option<String> {
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
    async fn parse_body(&self, article_src: &str) -> Option<String> {
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
        let urls = urls.into_iter().take(30).collect::<Vec<_>>();

        println!("Found {} article URLs. Fetching and parsing articles...", urls.len());
        let cnt: i32 = 0;

        for url in urls {
            cnt += 1;
            println!("Processing URL ({} of {}): {}", cnt, urls.len(), url);
            // if successfully parsed url, then continue, else skip to next url
            let article_src = match reqwest::get(&url).await {
                Ok(response) => match response.text().await {
                    Ok(text) => text,
                    Err(_) => continue,
                },
                Err(_) => continue,
            };

            if let Some(headline) = self.parse_headline(&article_src).await {
                if let Some(body) = self.parse_body(&article_src).await {
                    articles.push((headline, body));
                }
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
        let html = r#"
            <article class="story-1">
                <div class="m">
                    <a href="https://www.foxnews.com/article1">Article 1</a>
                </div>
            </article>
            <article class="story-2">
                <div class="m">
                    <a href="https://www.foxnews.com/article2">Article 2</a>
                </div>
            </article>
        "#;
        let parser = FoxParser::new(html);
        let urls = parser.parse_news_urls();
        assert_eq!(urls.len(), 2);
        assert_eq!(urls[0], "https://www.foxnews.com/article1");
        assert_eq!(urls[1], "https://www.foxnews.com/article2");
    }

    #[tokio::test]
    async fn test_parse_headline_and_body() {
        let test_article_html = r#"
            <html>
                <body>
                    <div class="article-meta-upper">
                        <h1 class="headline">Test Headline</h1>
                    </div>
                    <div class="article-body">
                        <p>Paragraph 1.</p>
                        <p>Paragraph 2.</p>
                    </div>
                </body>
        "#;
        let parser = FoxParser::new("");
        let headline = parser.parse_headline(test_article_html).await;
        let body = parser.parse_body(test_article_html).await;
        assert_eq!(headline, Some("Test Headline".to_string()));
        assert_eq!(body, Some("Paragraph 1.\n\nParagraph 2.".to_string()));
    }
}