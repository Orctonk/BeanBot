
extern crate glob;
use glob::glob;

use markov::*;
use std::path::Path;
use std::fs;
use serenity::client::Context;
use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use std::fs::DirEntry;
use std::io::Error;


pub struct ChainMap;

//Nyckeltyp för SharedMap
impl TypeMapKey for ChainMap{
    type Value = HashMap<String, Chain<String>>;
}

pub async fn init_chain_file(chain_name: &str, order: usize) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let path_string = format!("markov/chains/{}.chain", chain_name);
    let path = Path::new(path_string.as_str());
    if !path.exists() {
        eprintln!("Generating new chain ({}.chain)", chain_name);
        let mut chain: Chain<String> = Chain::of_order(order);
        chain.feed_file(Path::new(format!("markov/texts/{}.txt", chain_name).as_str()))?.save(path)?;
    }
    Ok(())
}

pub async fn init_chain_map(client: &Context) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut chain_map: HashMap<String, Chain<String>> = HashMap::new();
    let paths: glob::Paths = glob("markov/chains/*.chain").expect("Failed to read glob pattern");
    for entry in paths {
        //INSERT LOADED CHAIN INTO MAP
        let path_buf = entry?;
        chain_map.insert(path_buf.as_path().file_name().unwrap().to_os_string().into_string().unwrap(), Chain::load(path_buf.as_path())?);
    }
    let mut data = client.data.write().await;
    data.insert::<ChainMap>(chain_map);
    Ok(())
}