use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct QueryObjects {
    synonyms: Vec<Synonym>
}
#[derive(Deserialize, Debug)]
struct Synonym {
    word: String, 
    score: usize,
    tags: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let word = "today";
    let link: &str = &format!("http://api.datamuse.com/words?ml={}", word);
    let request = reqwest::get(link).await?.text().await?;
    let end_cutoff = &request.len() - 1;
    let request_ref = &request[1..end_cutoff];
    let json = serde_json::from_str::<QueryObjects>(&request_ref);

    println!("{:#?}", request);
    println!("{:#?}", json);
    // match json {
    //     Err(err) => println!("wow")
    // }

    Ok(())
}