use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::id::UserId; 
use serenity::utils::Colour;
use serenity::framework::standard::{
    CommandResult,
    Args,
    macros::command,
    macros::group,
};
use rand::Rng;

use crate::backend::currency::*;

const DB_ERROR_MESSAGE: &str = "Someone spilled beans on the servers. Please try again in a bit!";
const DAILY_BEAN_AMOUNT: u32 = 1;
const WEEKLY_BEAN_AMOUNT: u32 = 5;
const MONTHLY_BEAN_AMOUNT: u32 = 10;
const YEARLY_BEAN_AMOUNT: u32 = 50;

#[group]
#[prefix = "beans"]
#[description = "A group with commands related to the bean currency"]
#[summary = "Bean currency commands"]
#[commands(gimme,showme,give,eat,daily,weekly,monthly,yearly,beanmaster,beanboard)]
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
    let amount = args.single::<u32>().unwrap_or(5);
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
    if upper == 0 {
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

#[command]
#[description = "Get your daily bean allowance"]
#[max_args(0)]
pub async fn daily(ctx: &Context, msg: &Message) -> CommandResult {
    let userid = msg.author.id.0;
    match claim_daily(userid,DAILY_BEAN_AMOUNT) {
        Err(CurrencyError::NotReadyYet(timeleft)) => {
            let hours = timeleft.num_hours();
            let mins = timeleft.num_minutes() - hours * 60;
            let secs = timeleft.num_seconds() - timeleft.num_minutes() * 60;
            msg.channel_id.say(&ctx.http, &format!("You have already had your daily beans, try again in `{:?}h {:?}m {:?}s`",hours,mins,secs )).await?
        },
        Ok(_) => msg.channel_id.say(&ctx.http, &format!("You've claimed your daily `{:?}` beans",DAILY_BEAN_AMOUNT)).await?,
        Err(_) => msg.channel_id.say(&ctx.http, DB_ERROR_MESSAGE).await?
    };
    Ok(())
}

#[command]
#[description = "Get your weekly bean allowance"]

#[max_args(0)]
pub async fn weekly(ctx: &Context, msg: &Message) -> CommandResult {
    let userid = msg.author.id.0;
    match claim_weekly(userid,WEEKLY_BEAN_AMOUNT) {
        Err(CurrencyError::NotReadyYet(timeleft)) => {
            let days = timeleft.num_days();
            let hours = timeleft.num_hours() - days*24;
            let mins = timeleft.num_minutes() - timeleft.num_hours() * 60;
            let secs = timeleft.num_seconds() - timeleft.num_minutes() * 60;
            msg.channel_id.say(&ctx.http, &format!("You have already had your weekly beans, try again in `{:?} days, {:?}h {:?}m {:?}s`",days,hours,mins,secs )).await?
        },
        Ok(_) => msg.channel_id.say(&ctx.http, &format!("You've claimed your weekly `{:?}` beans",WEEKLY_BEAN_AMOUNT)).await?,
        Err(_) => msg.channel_id.say(&ctx.http, DB_ERROR_MESSAGE).await?
    };
    Ok(())
}

#[command]
#[description = "Get your monthly bean allowance"]
#[max_args(0)]
pub async fn monthly(ctx: &Context, msg: &Message) -> CommandResult {
    let userid = msg.author.id.0;
    match claim_monthly(userid,MONTHLY_BEAN_AMOUNT) {
        Err(CurrencyError::NotReadyYet(timeleft)) => {
            let days = timeleft.num_days();
            let hours = timeleft.num_hours() - days*24;
            let mins = timeleft.num_minutes() - timeleft.num_hours() * 60;
            let secs = timeleft.num_seconds() - timeleft.num_minutes() * 60;
            msg.channel_id.say(&ctx.http, &format!("You have already had your monthly beans, try again in `{:?} days, {:?}h {:?}m {:?}s`",days,hours,mins,secs )).await?
        },
        Ok(_) => msg.channel_id.say(&ctx.http, &format!("You've claimed your monthly `{:?}` beans",MONTHLY_BEAN_AMOUNT)).await?,
        Err(_) => msg.channel_id.say(&ctx.http, DB_ERROR_MESSAGE).await?
    };
    Ok(())
}

#[command]
#[description = "Get your yearly bean allowance"]
#[max_args(0)]
pub async fn yearly(ctx: &Context, msg: &Message) -> CommandResult {
    let userid = msg.author.id.0;
    match claim_yearly(userid,YEARLY_BEAN_AMOUNT) {
        Err(CurrencyError::NotReadyYet(timeleft)) => {
            let days = timeleft.num_days();
            let hours = timeleft.num_hours() - days*24;
            let mins = timeleft.num_minutes() - timeleft.num_hours() * 60;
            let secs = timeleft.num_seconds() - timeleft.num_minutes() * 60;
            msg.channel_id.say(&ctx.http, &format!("You have already had your yearly beans, try again in `{:?} days, {:?}h {:?}m {:?}s`",days,hours,mins,secs )).await?
        },
        Ok(_) => msg.channel_id.say(&ctx.http, &format!("You've claimed your yearly `{:?}` beans",YEARLY_BEAN_AMOUNT)).await?,
        Err(_) => msg.channel_id.say(&ctx.http, DB_ERROR_MESSAGE).await?
    };
    Ok(())
}

#[command]
#[description = "Call upon the Bean Master, the Master of the Beans, the boi with the most beans."]
#[max_args(0)]
pub async fn beanmaster(ctx: &Context, msg: &Message) -> CommandResult {
    //let phraces = vec!["The Bean Master is {:?}"];
    let bean_master = get_highest_balance(); 
    match bean_master {
        Ok(bean) =>{
            let master_id = UserId (bean);
            let master_user = master_id.to_user(ctx).await;
            match master_user {
                Ok(master) => msg.channel_id.say(&ctx.http, &format!("The Bean Master is {:?}", master.name)).await?,
                Err(_) => msg.channel_id.say(&ctx.http, "There is no beanmaster, you are free").await?
            }
    },

        Err(_) => msg.channel_id.say(&ctx.http, DB_ERROR_MESSAGE).await?
    };
    Ok(())
}

#[command]
#[description = "See the score board of beans!"]
#[max_args(0)]
pub async fn beanboard(ctx: &Context, msg: &Message) -> CommandResult {
    //let phraces = vec!["The Bean Master is {:?}"];
    let mut users = Vec::new();
    let bean_scores = get_scores(); 
    match bean_scores {
        Ok(beans) =>{
            for (bean, amount) in beans {
                let user_id = UserId (bean);
                let user = user_id.to_user(ctx).await;
                match user {
                    Ok(new) => users.push((new.name, amount, false)),
                    Err(_) => {}
                };
            }
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("***Bean Board!***");
                    e.color(Colour(16750123));
                    e.fields(users);
                    e
                })
            }).await?;
            Ok(())
        },
        Err(_) => {
            msg.channel_id.say(&ctx.http, DB_ERROR_MESSAGE).await?;
            Ok(())
        }
    }
}