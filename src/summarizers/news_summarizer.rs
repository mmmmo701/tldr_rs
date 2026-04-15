//! News Summarizer Module
//!
//! This module defines the `SummarizedNews` struct, which encapsulates a news article's headline and body text, along with a method to generate a summary using the Ollama API. The `summarize` method constructs a prompt for the language model and updates the summary field with the generated summary or an error message if the summarization fails.

use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

/// A struct representing a news article with its headline, body, and summary.
pub struct SummarizedNews {
    text: (String, String),
    summary: String,
    importance: i32,
}

impl Clone for SummarizedNews {
    fn clone(&self) -> Self {
        Self {
            text: (self.text.0.clone(), self.text.1.clone()),
            summary: self.summary.clone(),
            importance: self.importance,
        }
    }
}

impl SummarizedNews {
    /// Creates a new instance of `SummarizedNews` with the provided headline and body text.
    /// The summary is initialized to a default message indicating that it has not been summarized yet.
    /// # Arguments
    /// * `text` - A tuple containing the headline and body of the news article.
    pub fn new(text: (String, String)) -> Self {
        Self { text, summary: "not summarized yet".to_string(), importance: 0 }
    }

    /// Asynchronously generates a summary of the news article using the Ollama API.
    /// It constructs a prompt that includes the headline and body of the article, and then sends a generation request to the Ollama API. The summary field is updated with the response from the API or an error message if the request fails.
    pub async fn summarize(&mut self) {
        let (headline, body) = &self.text;
        let ollama = Ollama::default();
        let prompt = format!("Summarize the following news article into 80-100 words:\n\nHeadline: {}\n\nBody: {}", headline, body);
        
        let res = ollama.generate(GenerationRequest::new("llama3.2:3b".to_string(), prompt)).await;

        match res {
            Ok(summary) => { 
                self.summary = summary.response
            },
            Err(e) => { 
                self.summary = format!("Error during summarization: {}", e); 
            },
        };
    }

    /// Asynchronously generates an importance rating for the news article using the Ollama API.
    /// It constructs a prompt that asks the model to rate the importance of the article on a scale of 1 to 10, and then sends a generation request to the Ollama API. The importance field is updated with the parsed integer response from the API or set to 0 if there's an error.
    pub async fn get_importance_rating(&mut self) {
        let (headline, body) = &self.text;
        let ollama = Ollama::default();
        let prompt = format!("Rate the importance of the following news article to the general public on a scale of 1 to 10, where 1 is least important and 10 is most important:\n\nHeadline: {}\n\nBody: {}", headline, body);
        
        let res = ollama.generate(GenerationRequest::new("llama3.2:3b".to_string(), prompt)).await;

        match res {
            Ok(rating) => { 
                self.importance = rating.response.trim().parse::<i32>().unwrap_or(0);
            },
            Err(_) => { 
                self.importance = 0; // Default to 0 if there's an error
            },
        };
    }

    /// Getter method for the headline of the news article.
    pub fn get_headline(&self) -> String {
        self.text.0.clone()
    }

    /// Getter method for the body of the news article.
    pub fn get_body(&self) -> String {
        self.text.1.clone()
    }

    /// Getter method for the summary of the news article.
    pub fn get_summary(&self) -> String {
        self.summary.clone()
    }

    /// Getter method for the importance rating of the news article.
    pub fn get_importance(&self) -> i32 {
        self.importance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_summarize() {
        let test_headline = "Test Headline".to_string();
        let test_body = "This is a test body of the news article. It contains multiple sentences to simulate a real news article. The summarizer should be able to condense this information into a concise summary.".to_string();
        let mut news = SummarizedNews::new((test_headline, test_body));
        assert_eq!(news.summary, "not summarized yet".to_string());
        news.summarize().await;
    }

    #[tokio::test]
    async fn test_summarize2() {
        let test_headline = "Students in Stuco Rust met a big crab on the cut".to_string();
        let test_body = "A large crab was found on the cut. It was a big one.".to_string();
        let mut news = SummarizedNews::new((test_headline, test_body));
        assert_eq!(news.summary, "not summarized yet".to_string());
        news.summarize().await;
    }

    #[test]
    fn test_new() {
        let headline = "Breaking News".to_string();
        let body = "Article content here".to_string();
        let news = SummarizedNews::new((headline.clone(), body.clone()));
        assert_eq!(news.get_headline(), headline);
        assert_eq!(news.get_body(), body);
        assert_eq!(news.get_summary(), "not summarized yet");
    }

    #[test]
    fn test_get_headline() {
        let headline = "Test Headline".to_string();
        let body = "Test body".to_string();
        let news = SummarizedNews::new((headline.clone(), body));
        assert_eq!(news.get_headline(), headline);
    }

    #[test]
    fn test_get_body() {
        let headline = "Test Headline".to_string();
        let body = "Test body content".to_string();
        let news = SummarizedNews::new((headline, body.clone()));
        assert_eq!(news.get_body(), body);
    }

    #[test]
    fn test_get_summary() {
        let news = SummarizedNews::new(("Title".to_string(), "Body".to_string()));
        assert_eq!(news.get_summary(), "not summarized yet");
    }

    #[test]
    fn test_clone() {
        let news = SummarizedNews::new(("Headline".to_string(), "Body text".to_string()));
        let cloned = news.clone();
        assert_eq!(cloned.get_headline(), "Headline");
        assert_eq!(cloned.get_body(), "Body text");
        assert_eq!(cloned.get_summary(), "not summarized yet");
    }
}