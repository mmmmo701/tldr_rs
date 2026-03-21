use reqwest;
use scraper::{Html, Selector};

pub struct NewsWebsite {
    url: String,
}

impl NewsWebsite {
    pub fn from_url(my_url: &str) -> Self {
        Self {
            url: my_url.to_string(),
        }
    }

    pub async fn fetch_raw_main_page(&self) -> Result<String, Box<dyn std::error::Error>> {
        let body = reqwest::get(&self.url)
            .await?
            .text()
            .await?;
        Ok(body)
    }

    pub async fn get_news_urls(&self, raw_html: &str) -> Vec<String> {
        Vec::from([])       // TODO: change me
    }
}
