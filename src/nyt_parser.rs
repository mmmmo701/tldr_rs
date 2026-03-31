pub struct nyt_parser<'a> {
    src: &'a str,
}

//TODO: from news HTML, parse all links that starts with https://www.nytimes.com/YYYY/MM/DD/