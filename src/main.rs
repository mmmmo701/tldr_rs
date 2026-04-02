mod news_websites;
mod parsers;
use news_websites::NewsWebsite;

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

/// The main function initializes the asynchronous runtime, fetches news articles from both Fox News and New York Times, and prints their headlines and bodies to the console. It handles any errors that may occur during the fetching and parsing process.
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    println!("Fetching Fox news articles...");
    let fox_news_content = rt.block_on(get_foxnews()).unwrap();
    println!("Fetching NYT news articles...");
    let nyt_news_content = rt.block_on(get_nytimes()).unwrap();
    let all_news_content = [fox_news_content, nyt_news_content].concat();

    for (i, (headline, body)) in all_news_content.iter().enumerate() {
        println!("Article {}:", i + 1);
        println!("Headline: {}", headline);
        println!("Body: {}\n", body);
    }
}
