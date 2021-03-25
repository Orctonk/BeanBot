use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    Args,
    macros::command,
    macros::group,
};

use crate::backend::markov::*;
use markov::Chain;

#[group]
#[commands(beanverse)]
struct Markov;

#[command]
#[aliases(bv)]
pub async fn beanverse(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let result: Option<&ChainResult> = data.get::<BibleChain>();
    if let None = result{
        msg.channel_id.say(&ctx.http, "Could not generate beanverse").await?;
        return Ok(())
    }

    match generate_sentence(result.unwrap()) {
        Ok(tokens) => {
            let mut message: String = "".parse().unwrap();
            for string in tokens.iter(){
                message.push_str(string);
                message.push(' ');
            }
            msg.channel_id.say(&ctx.http, message).await?;
        },
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Could not generate beanverse").await?;
            ()
        },
    }
    Ok(())
}