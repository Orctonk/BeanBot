use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    Args,
    macros::command,
    macros::group,
};

use crate::backend::currency::*;

#[group]
#[prefix = "beans"]
#[description = "A group with commands related to the bean currency"]
#[summary = "Bean currency commands"]
#[commands(gimme,showme,give)]
#[default_command(showme)]
struct Currency;

#[command]
#[owners_only]
#[description = "Gives a specified amount of beans"]
#[usage = "[amount]"]
#[example = "100"]
#[min_args(0)]
#[max_args(1)]
pub async fn gimme(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let userid = msg.author.id.0;
    let amount = match args.single::<u32>() {
        Err(_) => 5,
        Ok(am) => am
    };
    add_beans(userid, amount);
    msg.channel_id.say(&ctx.http, "Here, have `".to_string() + &amount.to_string() +"` beans!").await?;
    Ok(())
}

#[command]
#[description = "Shows your current bean balance"]
#[max_args(0)]
pub async fn showme(ctx: &Context, msg: &Message) -> CommandResult {
    let userid = msg.author.id.0;
    let _ = match get_bean_balance(userid) {
        Err(why) => msg.channel_id.say(&ctx.http, "Failed to get beans: ".to_owned() + &why).await?,
        Ok(bal) => msg.channel_id.say(&ctx.http, "You have `".to_owned() + &bal.to_string() +"` beans").await?
    };
    Ok(())
}

#[command]
#[description = "Gives the mentioned user a specified amount of beans"]
#[usage = "@user amount"]
#[example = "@Beanlover 50"]
#[min_args(2)]
#[max_args(2)]
pub async fn give(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let userid = msg.author.id.0;
    let recipient = &msg.mentions[0];
    let _ = args.single::<String>(); // First arg is mention
    match args.single::<u32>(){
        Err(_) => 
            msg.channel_id.say(&ctx.http, "Invalid amount specified").await?,
        Ok(am) => {
            match transfer_beans(userid, recipient.id.0, am) {
                Err(why) => msg.channel_id.say(&ctx.http, "Failed to give beans: ".to_owned() + &why).await?,
                Ok(_) => msg.channel_id.say(&ctx.http, "Gave ".to_owned() + &recipient.name + " `" + &am.to_string() +"` beans").await?
            }
        }
    };

    Ok(())
}