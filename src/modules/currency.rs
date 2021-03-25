use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    Args,
    macros::command,
    macros::group,
};
use rand::Rng;

use crate::backend::currency::*;

const DB_ERROR_MESSAGE: &str = "Someone spilled beans on the servers. Please try again in a bit!";
#[group]
#[prefix = "beans"]
#[description = "A group with commands related to the bean currency"]
#[summary = "Bean currency commands"]
#[commands(gimme,showme,give,eat)]
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
    let _ = match add_beans(userid, amount) {
        Err(_) => msg.channel_id.say(&ctx.http, DB_ERROR_MESSAGE).await?,
        Ok(()) => msg.channel_id.say(&ctx.http, &format!("Here, have `{:?}` beans!",amount)).await?
    };
    
    Ok(())
}

#[command]
#[description = "Shows your current bean balance"]
#[max_args(0)]
pub async fn showme(ctx: &Context, msg: &Message) -> CommandResult {
    let userid = msg.author.id.0;
    let _ = match get_bean_balance(userid) {
        Err(_) => msg.channel_id.say(&ctx.http, DB_ERROR_MESSAGE).await?,
        Ok(bal) => msg.channel_id.say(&ctx.http, &format!("You have `{:?}` beans",bal)).await?
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
        Err(_) =>  msg.channel_id.say(&ctx.http, "Invalid amount specified").await?,
        Ok(am) => {
            match transfer_beans(userid, recipient.id.0, am) {
                Err(CurrencyError::InsufficientBalance) => msg.channel_id.say(&ctx.http, "You can't give beans you dont have!").await?,
                Ok(_) => msg.channel_id.say(&ctx.http, &format!("Gave {:?} `{:?}` beans",recipient.name,am)).await?,
                Err(_) => msg.channel_id.say(&ctx.http, DB_ERROR_MESSAGE).await?
            }
        }
    };

    Ok(())
}

#[command]
#[description = "Eat some beans"]
#[max_args(0)]
pub async fn eat(ctx: &Context, msg: &Message) -> CommandResult {
    let userid = msg.author.id.0;
    let max = match get_bean_balance(userid) {
        Err(_) => { 
            msg.channel_id.say(&ctx.http, DB_ERROR_MESSAGE).await?; 
            return Ok(()); 
        },
        Ok(amount) => amount
    };
    let upper = std::cmp::min(max, 10);
    if upper <= 0{
        msg.channel_id.say(&ctx.http, "You don't have any beans to eat!").await?;
        return Ok(());
    }
    let beans_eaten = rand::thread_rng().gen_range(1..(upper+1));
    match withdraw_beans(userid, beans_eaten){
        Err(CurrencyError::InsufficientBalance) => msg.channel_id.say(&ctx.http, "You don't have any beans to eat!").await?,
        Ok(_) => msg.channel_id.say(&ctx.http, &format!("Ate `{:?}` beans",beans_eaten)).await?,
        Err(_) => msg.channel_id.say(&ctx.http, DB_ERROR_MESSAGE).await?
    };

    Ok(())
}