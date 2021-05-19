use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    Args,
    macros::command,
    macros::group,
};
use rand::Rng;
//WTF is a crate? 
use crate::backend::currency::*;
use crate::backend::specialbeans::*;

//Gotta make that MONEYYYY
const JAR_COST: u32 = 10; 

#[group]
#[commands(buy, mybeans, about)]
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
            let id = get_random_id();
            let name =  add_special_bean(userid, id);
            match name {
                Err (_) => msg.channel_id.say(&ctx.http, &format!("Sorry, no beans for you!")).await?,
                Ok(bean) =>  msg.channel_id.say(&ctx.http, &format!("You bought a jar of beans and you got a `{:?}`!", bean)).await?
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

#[command]
#[description = "Prints facts about a special bean"]
#[min_args(0)]
pub async fn about(ctx: &Context, msg: &Message, args: Args) -> CommandResult{
    let name  = args.remains().unwrap_or("");
    let info = get_info_from_name(&name);
    match info {
        Err(_) => {
            msg.channel_id.say(&ctx.http, &format!("Could not find information about `{:?}`",name)).await?;
            return Ok(());
        }
        Ok((about, image)) => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(&format!("***About {:?}:***", name));
                    e.color(Colour(16750123));
                    e.thumbnail(image);
                    e.description(about);
                    e
                })
            }).await?;
            return Ok(());
        }
    };
}

// Function for getting a random bean ID from database
fn get_random_id() -> u32 {
    let mut final_weighted = Vec::new();
    let weighted_beans = get_all_beans();
    match weighted_beans{
        Ok(beans) => {
            for (id, weight) in beans {
                for i in 1..weight {
                    final_weighted.push(id)
                }
            }
        }
        Err(_) =>  println!("Failed to get random bean id")
    };
    let index = rand::thread_rng().gen_range(0..(final_weighted.len()));
    return final_weighted[index];
}

