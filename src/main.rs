use std::collections::HashMap;
use json::JsonValue;
use reqwest;
// use serde::Deserialize;

#[derive(serde::Deserialize)]
struct JSON { 
    value: String
}

async fn query_with_word(word: &str) -> Result<String, reqwest::Error> {
    let link: &str = &format!("http://api.datamuse.com/words?ml={}", word);
    let query = reqwest::get(link).await?;

    Ok(query.json::<String>().await?)
}

fn print_json(q: Result<String, reqwest::Error>) {
    match q {
        Ok(response) => println!("{:?}", response),
        Err(response) => println!("WARIO"),
    }
}

fn main() {
    let query = query_with_word("contrarian");
    print_json(query);
}
