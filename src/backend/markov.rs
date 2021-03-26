use markov::*;
use std::path::Path;
use std::error::Error;
use serenity::Client;
use serenity::client::Context;
use serenity::prelude::TypeMapKey;
use std::io::ErrorKind;


pub struct BibleChain;

pub type ChainResult = std::io::Result<Chain<String>>;

impl TypeMapKey for BibleChain{
    type Value = ChainResult;
}

pub fn generate_sentence(chain: &ChainResult) -> std::io::Result<Vec<String>>{
    let mut flag = true;
    let mut out: std::io::Result<Vec<String>> = Ok(Vec::new());
    let beans: Vec<&str> = vec!["beans", "bean", "wobean", "wobeans", "beansus", "heinz"];
    while flag{
        out = match chain {
            Ok(c) => {
                let genned = c.generate();
                for string in genned.iter(){
                    for pat in beans.iter(){
                        if string.to_lowercase().contains(pat){
                            flag = false;
                        }
                    }
                }
                Ok(genned)
            },
            Err(_) => {
                flag = false;
                Err(std::io::Error::new(ErrorKind::InvalidData, "No"))
            }
        }
    }
    return out;
}

pub async fn init_chain(client: &Context){
    let path = Path::new("bible.chain");
    if !path.exists(){
        eprintln!("{}", "Generating new bean chain");
        let mut chain = Chain::of_order(3);
        let fed_chain = chain.feed_file(Path::new("beanble.txt"));
        match fed_chain{
            Ok(bean) => {
                let _ = bean.save(path);
                ()
            },
            Err(_) => {
                eprintln!("{}", "Failed to generate new bean chain")
            },
        }
    }
    let mut data = client.data.write().await;
    data.insert::<BibleChain>(Chain::load(path));
}