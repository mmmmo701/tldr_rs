use reqwest;
use scraper::{Html, Selector};
use std::sync::LazyLock;

static HEADLINE_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("div.article-meta-upper h1.headline")
        .expect("Static Headline CSS selector is malformed")
});

static BODY_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("div.article-body p")
        .expect("Static Body CSS selector is malformed")
});

pub struct FoxParser<'a> {
    src: &'a str,
}

impl<'a> FoxParser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }

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

    async fn parse_headline(&self, url: &str) -> Option<String> {
        let article_src = reqwest::get(url).await.ok()?.text().await.ok()?;
        let document = Html::parse_document(&article_src);

        document.select(&HEADLINE_SELECTOR).next().map(|element| {
            element
                .text()
                .collect::<Vec<_>>()
                .concat()
                .trim()
                .to_string()
        })
    }

    async fn parse_body(&self, url: &str) -> Option<String> {
        let article_src = reqwest::get(url).await.ok()?.text().await.ok()?;
        let document = Html::parse_document(&article_src);

        let body_texts: Vec<String> = document
            .select(&BODY_SELECTOR)
            .map(|element| element.text().collect::<Vec<_>>().concat().trim().to_string())
            .collect();

        if body_texts.is_empty() {
            None
        } else {
            Some(body_texts.join("\n\n"))
        }
    }

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