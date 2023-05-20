use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Synonym {
    word: String, 
    score: usize,
    tags: Vec<String>,
}

async fn get_synonyms(of_word: &str) -> Result<Vec<Synonym>, reqwest::Error> {
    let link: &str = &format!("http://api.datamuse.com/words?ml={}", of_word);
    let query: Vec<Synonym> = reqwest::get(link).await?.json::<Vec<Synonym>>().await?;
    
    Ok(query)
}

async fn rewrite_sentence(sentence: &str) -> Result<String, reqwest::Error> {
    let mut replacing_words: Vec<String> = Vec::new();
    for word in sentence.split(' ') {
        // let synonym = &get_synonyms(word).await?[0].word;
        replacing_words.push(get_synonyms(word).await?[0].word.to_string())
    }    
    
    let rewritten: String = replacing_words.join(" ");
    Ok(rewritten)
}

fn parse_arg_sentence() {

}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {    
    let s: &str = "incredibly engaging presenter";
    println!("{}", rewrite_sentence(s).await?);

    Ok(())
}