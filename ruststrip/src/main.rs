use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Stripped {
   base: String
}

impl Stripped {
   async fn get(word: &String) -> Result<Self, ExitFailure> {
      let url = format!("http://159.203.4.190:43560/strip/{}", word);
      let url = Url::parse(&*url)?;
      let res = reqwest::get(url).await?.json::<Stripped>().await?;

      Ok(res)
   }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args: Vec<String> = env::args().collect();
    let mut rawword: String = "благови́дный".to_string();
    println!("There are {} arguments. First is {}", args.len(), args[0]);
    if args.len() < 2 {
        println!("Since you didn't specify a word, it has defaulted to благови́дный.");
    } else {
        rawword = args[1].clone();
    }

    let res = Stripped::get(&rawword).await?;
    println!("{} stripped form: {}", rawword, res.base);

    Ok(())
}
