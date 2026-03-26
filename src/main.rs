mod news_websites;
mod fox_parser;
use news_websites::NewsWebsite;

async fn get_foxnews() -> Result<(), Box<dyn std::error::Error>> {
    let fox_web = NewsWebsite::from_url("https://foxnews.com");
    match fox_web.fetch_raw_main_page().await {
        Ok(html) => {
            let fp = fox_parser::FoxParser::new(&html);
            let urls = fp.parse_top_articles().await;
            for (headline, body) in urls {
                println!("Headline: {}\nBody: {}\n\n==========================\n\n", headline, body);
            }
        },
        Err(e) => return Err(e), 
    }
    Ok(())
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(get_foxnews()).unwrap();
}
