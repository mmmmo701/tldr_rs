//! News Filter Module
//! This module defines the `NewsList` struct, which encapsulates a list of `SummarizedNews` items and provides methods to summarize all news articles, rate their importance, and filter them based on importance. The `summarize_all` method iterates through all news items and calls their `summarize` method, while the `rate_importance_all` method calls the `get_importance_rating` method for each news item. The `filter_by_importance` method sorts the news items by their importance rating in descending order and truncates the list to keep only the specified maximum number of news articles.

use super::news_summarizer::SummarizedNews;

/// A struct representing a list of summarized news articles, along with methods to summarize, rate importance, and filter the news based on importance.
pub struct NewsList {
    news: Vec<SummarizedNews>,
}

impl NewsList {
    /// Creates a new instance of `NewsList` with the provided vector of `SummarizedNews` items.
    /// # Arguments
    /// * `news` - A vector containing instances of `SummarizedNews` that represent individual news articles with their headlines, bodies, summaries, and importance ratings.
    pub fn new(news: Vec<SummarizedNews>) -> Self {
        Self { news }
    }

    /// Asynchronously generates summaries for all news articles in the list using the Ollama API. It iterates through each `SummarizedNews` item in the `news` vector and calls its `summarize` method, which sends a generation request to the Ollama API to obtain a summary for each article.
    pub async fn summarize_all(&mut self) {
        let mut cnt: i32 = 1;
        let len = self.news.len();
        let mynews: &mut Vec<SummarizedNews> = &mut self.news;
        for news_item in mynews {
            println!("Summarizing news article {} of {}...", cnt, len);
            news_item.summarize().await;
            cnt += 1;
        }
    }

    /// Asynchronously generates importance ratings for all news articles in the list using the Ollama API. It iterates through each `SummarizedNews` item in the `news` vector and calls its `get_importance_rating` method, which sends a generation request to the Ollama API to obtain an importance rating for each article on a scale of 1 to 10.
    pub async fn rate_importance_all(&mut self) {
        let mut cnt: i32 = 1;
        let len = self.news.len();
        let mynews: &mut Vec<SummarizedNews> = &mut self.news;
        for news_item in mynews {
            println!("Rating importance of news article {} of {}...", cnt, len);
            news_item.get_importance_rating().await;
            cnt += 1;
        }
    }

    /// Filters the news articles in the list based on their importance ratings. It sorts the `news` vector in descending order of importance using the `sort_by` method and then truncates the list to keep only the specified maximum number of news articles using the `truncate` method.
    /// # Arguments
    /// * `max_num_news` - An unsigned integer specifying the maximum number of news articles to keep in the list after filtering based on importance.
    pub fn filter_by_importance(&mut self, max_num_news: usize) {
        self.news
            .sort_by_key(|b| std::cmp::Reverse(b.get_importance()));
        self.news.truncate(max_num_news);
    }

    /// Prints the headlines and summaries of all news articles in the list to the console. It iterates through each `SummarizedNews` item in the `news` vector and prints its headline and summary using the `get_headline` and `get_summary` methods, respectively.
    pub fn print_news(&self) {
        for news_item in &self.news {
            println!("Headline: {}", news_item.get_headline());
            println!("Summary: {}\n", news_item.get_summary());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_summarize_all() {
        let mut news_list = NewsList::new(vec![]);
        news_list.summarize_all().await;
    }

    #[tokio::test]
    async fn test_rate_importance_all() {
        let mut news_list = NewsList::new(vec![]);
        news_list.rate_importance_all().await;
    }

    #[test]
    fn test_filter_by_importance_basic() {
        let mut news_list = NewsList::new(vec![]);
        news_list.filter_by_importance(5);
    }

    #[test]
    fn test_filter_by_importance() {
        let mut news_list = NewsList::new(vec![
            SummarizedNews::new(("Headline 1".to_string(), "Body 1".to_string())),
            SummarizedNews::new(("Headline 2".to_string(), "Body 2".to_string())),
            SummarizedNews::new(("Headline 3".to_string(), "Body 3".to_string())),
        ]);
        news_list.news[0].set_importance(3);
        news_list.news[1].set_importance(7);
        news_list.news[2].set_importance(5);
        news_list.filter_by_importance(2);
        assert_eq!(news_list.news.len(), 2);
        assert_eq!(news_list.news[0].get_importance(), 7);
        assert_eq!(news_list.news[1].get_importance(), 5);
    }

    #[test]
    fn test_filter_by_importance_with_zero_max() {
        let mut news_list = NewsList::new(vec![
            SummarizedNews::new(("Headline 1".to_string(), "Body 1".to_string())),
            SummarizedNews::new(("Headline 2".to_string(), "Body 2".to_string())),
            SummarizedNews::new(("Headline 3".to_string(), "Body 3".to_string())),
        ]);
        news_list.news[0].set_importance(3);
        news_list.news[1].set_importance(7);
        news_list.news[2].set_importance(5);
        news_list.filter_by_importance(0);
        assert_eq!(news_list.news.len(), 0);
    }
}
