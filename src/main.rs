use std::{
    collections::HashSet,
    env,
    panic,
};

mod modules;
use modules::saybeans::*;

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{
        StandardFramework,
        standard::macros::group,
        standard::Configuration,
    },
    http::Http,
    model::{event::ResumedEvent, channel::Message, gateway::Ready, gateway::Activity, id::UserId},
    prelude::*,
};

struct CommandHandler;

#[async_trait]
impl EventHandler for CommandHandler{
    async fn ready(&self, ctx: Context, _data_about_bot: Ready){
        ctx.set_activity(Activity::competing("Bean eating")).await;
        println!("Hello! I am ready to dispatch beans!");
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

#[group]
#[commands(saybeans)]
struct SayBeans;

#[tokio::main]
async fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file containing the bot token");
    } 
    let token = std::fs::read_to_string(std::path::Path::new(&args[1])).expect("Failed to open token file");

    let owners = vec![UserId(106865750614011904)].into_iter().collect();

    let framework = StandardFramework::new()
    .configure(|c: &mut Configuration| c
               .owners(owners)
               .prefix("!")
               .case_insensitivity(true)
                .on_mention(Some(UserId(354361968091594752))))
               .group(&SAYBEANS_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(CommandHandler)
        .await.expect("Error creating client");

    if let Err(why) = client.start().await{
        println!("Client error encountered: {:?}", why);
    }
}