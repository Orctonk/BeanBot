use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    Args,
    macros::command,
    macros::group,
};

use crate::backend::currency::*;
use crate::backend::specialbeans::*;

const DB_ERROR_MESSAGE: &str = "Someone spilled beans on the servers. Please try again in a bit!";
#[derive(Debug)]
struct OptionParseError;
#[derive(Debug)]
struct UpdateOptions {
    name: Option<String>,
    about: Option<String>,
    url: Option<String>,
    weight: Option<u32>,
}

#[group]
#[prefix = "beanadmin"]
#[commands(gimme,details,update,add)]
#[description = "Admin commands for special beans"]
#[summary = "Special Beans Admin"]
struct BeanAdmin;

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
#[description = "Prints details about a special bean"]
#[usage = "name"]
#[example = "stinky"]
#[min_args(1)]
#[owners_only]
pub async fn details(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    let name  = args.single_quoted::<String>().unwrap_or(String::from(""));
    let details = get_bean_full(&name);
    match details {
        Err(_) => {
            msg.channel_id.say(&ctx.http, &format!("Could not find information about `{:?}`",name)).await?;
            Ok(())
        }
        Ok((name, about, image, weight)) => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(&format!("***Details {:?}:***", name));
                    e.color(Colour(16750123));
                    e.description(&format!("Name: {}\nAbout: {}\nImage URL: {}\nWeight: {}",name,about,image,weight));
                    e
                })
            }).await?;
            Ok(())
        }
    }
}

#[command]
#[description = "Updates details about a special bean"]
#[usage = "name newname|- about|- url|- weight|-"]
#[example = "stinky stinker - - 5"]
#[min_args(1)]
#[owners_only]
pub async fn update(ctx: &Context, msg: &Message,mut args: Args) -> CommandResult{
    let name = args.single_quoted::<String>().unwrap_or("".to_string());
    let opt = match parse_args(args) {
        Err(_) => {
            msg.channel_id.say(&ctx.http, &format!("Failed to parse options")).await?;
            return Ok(())
        },
        Ok(val) => val
    };
    let details = match get_bean_full(&name) {
        Err(_) => {
            msg.channel_id.say(&ctx.http, &format!("Could not find information about `{:?}`",name)).await?;
            return Ok(())
        }
        Ok(det) => det
    };

    let new_name = opt.name.unwrap_or(details.0);
    let about = opt.about.unwrap_or(details.1);
    let url = opt.url.unwrap_or(details.2);
    let weight = opt.weight.unwrap_or(details.3);
    match update_special_bean(
        name.as_str(), 
        new_name.as_str(), 
        about.as_str(),
        url.as_str(),
        weight) {
        Err(_) => {
            msg.channel_id.say(&ctx.http, &format!("Failed to update bean")).await?;
            Ok(())
        },
        Ok(_) => {
            msg.channel_id.say(&ctx.http, &format!("Bean updated!")).await?;
            Ok(())
        }
    }
}

#[command]
#[description = "Add a special bean"]
#[usage = "name about url weight"]
#[example = "newbean \"this is a new bean\" newbean.org 5"]
#[min_args(1)]
#[owners_only]
pub async fn add(ctx: &Context, msg: &Message,args: Args) -> CommandResult{
    let opt = match parse_args(args) {
        Err(_) => {
            msg.channel_id.say(&ctx.http, &format!("Failed to parse options")).await?;
            return Ok(())
        },
        Ok(val) => val
    };
    // Is there a better way to do this? Yes
    // Do I know how to? No
    match opt {
        UpdateOptions {name: None, about: _, url: _, weight: _ } | 
        UpdateOptions {name: _, about: None, url: _, weight: _ } | 
        UpdateOptions {name: _, about: _, url: None, weight: _ } | 
        UpdateOptions {name: _, about: _, url: _, weight: None } => {
            msg.channel_id.say(&ctx.http, &format!("Failed to parse options")).await?;
            Ok(())
        }
        UpdateOptions {name: Some(name), about: Some(about), url: Some(url), weight: Some(weight)} => {
            match create_special_bean(name.as_str(), about.as_str(), url.as_str(), weight) {
                Err(_) => {
                    msg.channel_id.say(&ctx.http, &format!("Failed to insert bean")).await?;
                    Ok(())
                },
                Ok(_) => {
                    msg.channel_id.say(&ctx.http, &format!("Bean added!")).await?;
                    Ok(())
                }
            }
        }
    }
}

fn parse_args(mut args: Args) -> Result<UpdateOptions,OptionParseError> {
    Ok(UpdateOptions {
        name: match args.single_quoted::<String>() {
            Err(_) => return Err(OptionParseError),
            Ok(val) => match val.as_str() {
                "-" => None,
                _ => Some(val)
            }
        },
        about: match args.single_quoted::<String>() {
            Err(_) => return Err(OptionParseError),
            Ok(val) => match val.as_str() {
                "-" => None,
                _ => Some(val)
            }
        },
        url: match args.single_quoted::<String>() {
            Err(_) => return Err(OptionParseError),
            Ok(val) => match val.as_str() {
                "-" => None,
                _ => Some(val)
            }
        },
        weight: match args.single_quoted::<String>() {
            Err(_) => return Err(OptionParseError),
            Ok(val) => match val.as_str() {
                "-" => None,
                _ => match val.parse::<u32>() {
                    Err(_) => return Err(OptionParseError),
                    Ok(weight) => Some(weight)
                }
            }
        }
    })
}