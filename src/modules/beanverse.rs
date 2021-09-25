use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    macros::group,
};

use crate::backend::markov::*;

#[group]
#[description = "Create a bean related bible verse"]
#[summary = "Make bean verses"]
#[commands(beanverse)]
struct BeanVerse;

#[command]
#[description = "Prints a markov-chain generated bible verse (but with beans)"]
#[max_args(0)]
#[usage = ""]
#[aliases(bv)]
pub async fn beanverse(ctx: &Context, msg: &Message) -> CommandResult {
    match generate_sentence(ctx).await {
        //If sentence could be generated, combine tokens into string
        Ok(tokens) => {
            let mut message: String = "".parse().unwrap();
            for string in tokens.iter(){
                message.push_str(string);
                message.push(' ');
            }
            msg.channel_id.say(&ctx.http, message).await?;
        },
        Err(s) => {
            msg.channel_id.say(&ctx.http, s.as_str()).await?;

        },
    }
    Ok(())
}

async fn generate_sentence(client: &Context) -> std::result::Result<Vec<String>, String> {
    //Create vector of words which the generated sentence must contain to be a valid bean verse.
    let data = client.data.read().await;
    let beanble_chain = match data.get::<ChainMap>() {
        Some(map) => match map.get("beanble.chain") {
            Some(c) => c,
            _ => {return Err("No chain in chain map".to_string())}
        },
        _ => {return Err("No chain map in hash table".to_string())}
    };


    let beans: Vec<&str> = vec!["beans", "bean", "wobean", "wobeans", "beansus", "heinz"];
    loop {
        let tokens = beanble_chain.generate();
        if sentence_contains_words(&tokens, &beans) {
            return Ok(tokens);
        }
    }

}

fn sentence_contains_words(sentence: &[String], words: &[&str]) -> bool {
    for sentence_part in sentence.iter() {
        for word in words.iter() {
            if sentence_part.to_lowercase().contains(word) {
                //If bean word is found, break loop and return the generated sentance
                return true;
            }
        }
    }
    false
}