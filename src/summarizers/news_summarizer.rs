use ollama_rs:: Ollama;

pub struct SummerizedNews {
    text: (String, String),
    summary: String,
}

impl SummerizedNews {
    pub fn new(text: (String, String)) -> Self {
        Self { text, summary: "not summarized yet".to_string() }
    }

    pub fn summarize(&self, ollama: Ollama) {
        let (headline, body) = &self.text;

        let prompt = "Summarize the following news article into 80-100 words:\n\nHeadline: {}\n\nBody: {}";
        prompt = format!(prompt, headline, body);

        match ollama.generate(&prompt) {
            Ok(summary) => { 
                self.summary = summary 
            },
            Err(e) => { 
                self.summary = "Summary generation failed.".to_string() 
            },
        }
    }

    pub fn getSummary(&self) -> String {
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
        let news = SummerizedNews::new((test_headline, test_body));
        assert_eq!(news.summary, "not summarized yet".to_string());
        news.summarize(Ollama::new("news-summarizer"));
    }
}