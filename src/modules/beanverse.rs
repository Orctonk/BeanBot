use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    Args,
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
    //Aquire lock for the shared map
    let data = ctx.data.read().await;
    let result: Option<&ChainResult> = data.get::<BibleChain>();
    if let None = result{
        msg.channel_id.say(&ctx.http, "Could not generate beanverse. No chain").await?;
        return Ok(())
    }

    match generate_sentence(result.unwrap()) {
        //If sentence could be generated, combine tokens into string
        Ok(tokens) => {
            let mut message: String = "".parse().unwrap();
            for string in tokens.iter(){
                message.push_str(string);
                message.push(' ');
            }
            msg.channel_id.say(&ctx.http, message).await?;
        },
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Could not generate beanverse. Backends fault").await?;
            ()
        },
    }
    Ok(())
}