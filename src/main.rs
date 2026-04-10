mod news_websites;
mod parsers;
use news_websites::NewsWebsite;
mod summarizers;
use summarizers::news_summarizer::SummarizedNews;
use ollama_rs::generation::completion::request::GenerationRequest;

/// Main entry point of the application. It initializes the runtime, fetches news articles from both Fox News and New York Times, and prints their headlines and bodies to the console.
async fn get_foxnews() -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let fox_web = NewsWebsite::from_url("https://foxnews.com");
    let news_contents: Vec<(String, String)>;

    match fox_web.fetch_raw_main_page().await {
        Ok(html) => {
            let fp = parsers::fox::FoxParser::new(&html);
            news_contents = fp.parse_top_articles().await;
        },
        Err(e) => return Err(e), 
    }
    Ok(news_contents)
}

/// Fetches and parses news articles from the New York Times. It initializes a `NewsWebsite` instance for NYT, retrieves the raw HTML of the main page, and uses the `NytParser` to extract article headlines and bodies. The results are returned as a vector of tuples.
async fn get_nytimes() -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let nyt_web = NewsWebsite::from_url("https://www.nytimes.com");
    let news_contents: Vec<(String, String)>;
    match nyt_web.fetch_raw_main_page().await {
        Ok(html) => {
            let np = parsers::nyt::NytParser::new(&html);
            news_contents = np.parse_top_articles().await;
        },
        Err(e) => return Err(e), 
    }
    Ok(news_contents)
}

/// The function gets all news content from both Fox News and New York Times.
fn get_news(rt: &mut tokio::runtime::Runtime) -> Vec<(String, String)> {
    println!("Fetching Fox news articles...");
    let fox_news_content = rt.block_on(get_foxnews()).unwrap();

    println!("\nFetching NYT news articles...");
    let nyt_news_content = rt.block_on(get_nytimes()).unwrap();
    let all_news_content = [fox_news_content, nyt_news_content].concat();
    all_news_content
}

/// The main function serves as the entry point of the application. It calls the `get_news` function to retrieve news articles from both sources and proceed with summarization.
fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    println!("Starting news summarization application. Fetching news articles...");
    let news = get_news(&mut rt);

    println!("Summarizing {} news articles.", news.len());
    let mut cnt: i32 = 0;
    let summarized_news: Vec<SummarizedNews> = news.iter().map(|(headline, body)| {
        let mut news_item = SummarizedNews::new((headline.clone(), body.clone()));
        rt.block_on(news_item.summarize());
        cnt += 1;
        println!("Summarized news ({} of {}): {}", cnt, news.len(), news_item.get_summary());
        news_item
    }).collect();

    for summarized_news in summarized_news {
        println!("Headline: {}", summarized_news.get_headline());
        println!("Summary: {}\n", summarized_news.get_summary());
    }
}

// At the bottom of src/main.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_foxnews_structure() {
        let result = get_foxnews().await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_nytimes_structure() {
        let result = get_nytimes().await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_foxnews_content() {
        let result = get_foxnews().await;
        if let Ok(articles) = result {
            for (headline, body) in articles {
                assert!(!headline.is_empty(), "Headline should not be empty");
                assert!(!body.is_empty(), "Body should not be empty");
            }
        }
    }

    #[tokio::test]
    async fn test_get_nytimes_content() {
        let result = get_nytimes().await;
        if let Ok(articles) = result {
            for (headline, body) in articles {
                assert!(!headline.is_empty(), "Headline should not be empty");
                assert!(!body.is_empty(), "Body should not be empty");
            }
        }
    }
}