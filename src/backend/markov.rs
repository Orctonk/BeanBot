use markov::*;
use std::path::Path;
use serenity::client::Context;
use serenity::prelude::TypeMapKey;
use std::io::ErrorKind;
use std::io::Error;
use serenity::Result;


pub struct BibleChain;

pub type ChainResult = std::io::Result<Chain<String>>;

//Nyckeltyp för SharedMap
impl TypeMapKey for BibleChain{
    type Value = ChainResult;
}

pub fn generate_sentence(chain: &ChainResult) -> std::io::Result<Vec<String>>{
    let mut flag = true;
    let mut out: std::io::Result<Vec<String>> = Ok(Vec::new());
    //Create vector of words which the generated sentance must contain to be a valid beanverse.
    let beans: Vec<&str> = vec!["beans", "bean", "wobean", "wobeans", "beansus", "heinz"];
    while flag{
        out = match chain {
            Ok(c) => {
                //Generate a sentence and check for bean words
                let tokens = c.generate();
                'beans: for string in tokens.iter(){
                    for pat in beans.iter(){
                        if string.to_lowercase().contains(pat){
                            //If bean word is found, break loop and return the generated sentance
                            flag = false;
                            break 'beans;
                        }
                    }
                }
                Ok(tokens)
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
    //Loads a chain to be used with the bot or creates a new chain if none was found
    let path = Path::new("bible.chain");
    let loaded_chain: ChainResult;
    if !path.exists(){
        eprintln!("{}", "Generating new bean chain");
        let mut chain = Chain::of_order(3);
        let fed_chain = chain.feed_file(Path::new("beanble.txt"));
        if let Ok(bean) = fed_chain{
            let _ = bean.save(path);
            loaded_chain = Ok(chain);
        } else{
            loaded_chain = Err(Error::from(ErrorKind::Other));
            eprintln!("{}", "Failed to generate new bean chain");
        }
    } else{
        eprintln!("Loading existing chain");
        loaded_chain = Chain::load(path);
    }
    //Store loaded chain in clients SharedMap
    let mut data = client.data.write().await;
    data.insert::<BibleChain>(loaded_chain);
}