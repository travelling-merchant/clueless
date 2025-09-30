pub mod quotes {
    use serde::Deserialize;

    #[derive(PartialEq, Deserialize, Clone)]
    pub struct Entry {
        pub text: String,
        pub author: String,
        pub theme: String,
        pub probability: u8,
    }
    #[derive(Deserialize)]
    pub struct QuotesFile {
        pub quote: Vec<Entry>,
    }
    pub fn get_quotes() -> Vec<Entry> {
        let quotes_from_file = include_str!("../quotes.toml");
        toml::from_str::<QuotesFile>(quotes_from_file)
            .map(|possible_quote| possible_quote.quote)
            .unwrap_or_else(|_| {
                vec![Entry {
                    text: "No Quote Found".into(),
                    author: "No just No".into(),
                    theme: "No".into(),
                    probability: 0,
                }]
            })
            .iter()
            .cloned()
            .collect()
    }
}
