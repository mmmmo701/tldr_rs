mod scraper;
use scraper::NewsWebsite;

#[tokio::main] // 1. Necessary to run async code in main
async fn main() {
    let n = NewsWebsite::from_url("https://foxnews.com");

    match n.fetch_raw_main_page().await {
        Ok(html) => println!("The content is:\n{}", html),
        Err(e) => eprintln!("Some error occurred: {}", e), // 3. Use eprintln for errors
    }
}