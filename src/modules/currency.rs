use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    macros::group,
};

use crate::backend::currency::*;

#[group]
#[prefix = "beans"]
#[commands(gimme,showme)]
struct Currency;

#[command]
pub async fn gimme(ctx: &Context, msg: &Message) -> CommandResult {
    let userid = msg.author.id.0;
    add_beans(userid, 5);
    msg.channel_id.say(&ctx.http, "Here, have 5 beans!").await?;
    Ok(())
}

#[command]
pub async fn showme(ctx: &Context, msg: &Message) -> CommandResult {
    let userid = msg.author.id.0;
    let _ = match get_bean_balance(userid) {
        Err(why) => msg.channel_id.say(&ctx.http, "Failed to get beans ".to_owned() + &why).await?,
        Ok(bal) => msg.channel_id.say(&ctx.http, "You have ".to_owned() + &bal.to_string() +" beans").await?
    };
    Ok(())
}