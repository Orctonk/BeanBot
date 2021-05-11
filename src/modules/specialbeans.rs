use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    macros::group,
};
//WTF is a crate? 
use crate::backend::currency::*;
use crate::backend::specialbeans::*;

//Gotta make that MONEYYYY
const JAR_COST: u32 = 10; 

#[group]
#[commands(buy, mybeans)]
#[description = "Commands related to special beans"]
#[summary = "Special Beans"]
struct SpecialBeans;

#[command]
#[description = "Buys a jar of beans"]
#[min_args(0)]
pub async fn buy(ctx: &Context, msg: &Message) -> CommandResult {
    let userid = msg.author.id.0; 
    match withdraw_beans(userid, JAR_COST)  {
        Err (_) => msg.channel_id.say(&ctx.http, &format!("You don't have enought cum... I mean BEANS")).await?,
        Ok(_) =>  {
            match add_special_bean(userid, 2) {
                Err (_) => msg.channel_id.say(&ctx.http, &format!("Sorry, no beans for you!")).await?,
                Ok(_) =>  msg.channel_id.say(&ctx.http, &format!("You bought a jar of beans!")).await?
            }
        }
    };
    return Ok(());
}

#[command]
#[description = "Shows the user their special beans"]
#[min_args(0)]
pub async fn mybeans(ctx: &Context, msg: &Message) -> CommandResult {
    let userid = msg.author.id.0;
    let my_beans = get_special_beans(userid);
    match my_beans{
        Ok(beans) => {
            let beans_mapped : Vec<(String, u32, bool)> = beans.into_iter().map(|b| (b.0, b.1, false)).collect();
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("***Your Beans:***");
                    e.color(Colour(16750123));
                    e.fields(beans_mapped);
                    e
                })
            }).await?;
            return Ok(());
            
        }
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Could not find your beans :(").await?;
            return Ok(());
        }
    }
}