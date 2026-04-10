use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

pub struct SummarizedNews {
    text: (String, String),
    summary: String,
}

impl SummarizedNews {
    pub fn new(text: (String, String)) -> Self {
        Self { text, summary: "not summarized yet".to_string() }
    }

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

    pub fn get_headline(&self) -> String {
        self.text.0.clone()
    }

    pub fn get_body(&self) -> String {
        self.text.1.clone()
    }

    pub fn get_summary(&self) -> String {
        self.summary.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summarize() {
        let test_headline = "Test Headline".to_string();
        let test_body = "This is a test body of the news article. It contains multiple sentences to simulate a real news article. The summarizer should be able to condense this information into a concise summary.".to_string();
        let news = SummarizedNews::new((test_headline, test_body));
        assert_eq!(news.summary, "not summarized yet".to_string());
        news.summarize();
    }
}