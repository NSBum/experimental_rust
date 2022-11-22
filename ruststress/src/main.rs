use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
struct Stressed {
   version: i8,
   stressed: String
}

impl Stressed {
   async fn get(sentence: &String) -> Result<Self, ExitFailure> {
    let mut map = HashMap::new();

    map.insert("sentence", sentence);

    let url = "http://159.203.4.190:43560/stress/v2";
    let url = Url::parse(&*url)?;
    let client = reqwest::Client::new();
    let res = client.post(url)
        .json(&map)
        .send()
        .await?
        .json::<Stressed>()
        .await?;

    Ok(res)
   }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args: Vec<String> = env::args().collect();
    let mut rawsentence: String = "Это моя собака.".to_string();

    if args.len() < 1 {
        println!("Since you didn't specify a sentence, it has defaulted to Это моя собака.");
    } else {
        rawsentence = args[1].clone();
    }

    let res = Stressed::get(&rawsentence).await?;
    println!("{}", res.stressed);

    Ok(())
}
