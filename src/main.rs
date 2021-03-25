use std::{
    collections::HashSet,
    env,
    panic,
};

mod backend;
use backend::currency::*;

mod modules;
use modules::currency::*;
use modules::showmebeans::*;
use modules::markov::*;

use serenity::{
    async_trait,
    framework::{
        StandardFramework,
        standard::Configuration,
    },
    http::Http,
    model::{event::ResumedEvent, gateway::Ready, gateway::Activity},
    prelude::*,
};
use markov::Chain;
use crate::backend::markov::init_chain;

struct CommandHandler;

#[async_trait]
impl EventHandler for CommandHandler{
    async fn ready(&self, ctx: Context, _data_about_bot: Ready){
        create_wallet_table();
        init_chain(&ctx).await;
        ctx.set_activity(Activity::listening("Quilla - Beans Beans Beans")).await;
        println!("Hello! I am ready to dispatch beans!");
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

#[tokio::main]
async fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file containing the bot token");
    } 
    let token = std::fs::read_to_string(std::path::Path::new(&args[1])).expect("Failed to open token file");

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
    .configure(|c: &mut Configuration| c
        .owners(owners)
        .prefix("!")
        .case_insensitivity(true)
        .on_mention(Some(bot_id)))
        .group(&CURRENCY_GROUP)
        .group(&SHOWMEBEANS_GROUP)
        .group(&MARKOV_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(CommandHandler)
        .await.expect("Error creating client");

    if let Err(why) = client.start().await{
        println!("Client error encountered: {:?}", why);
    }
}