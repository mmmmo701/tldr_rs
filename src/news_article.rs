pub struct news_article<'a> {
    url: String,
    headline: String,
    body: &'a str,
}