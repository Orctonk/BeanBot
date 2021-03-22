use std::env;
use std::panic;
mod dispatcher;
mod module;

use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateMessage},
    model::{channel::Message, gateway::Ready, id::ChannelId, gateway::Activity},
    prelude::*,
    utils::Colour,
};

struct CommandHandler{
    dispatcher: dispatcher::Dispatcher,
}

#[async_trait]
impl EventHandler for CommandHandler{
    async fn message(&self, ctx: Context, new_message: Message){
        if new_message.mentions_me(&ctx.http).await.unwrap_or(false) || new_message.content.starts_with("<beans>"){
            self.dispatcher.dispatch(ctx, &new_message, &new_message.content).await;
        }
    }

    async fn ready(&self, ctx: Context, _data_about_bot: Ready){
        ctx.set_activity(Activity::competing("Bean eating")).await;
        println!("Hello! I am ready to dispatch beans!");
    }
}

#[tokio::main]
async fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file containing the bot token");
    } 
    let token = std::fs::read_to_string(std::path::Path::new(&args[1])).expect("Failed to open token file");

    let cmd_handler = CommandHandler {dispatcher: dispatcher::Dispatcher};
    let mut client = Client::builder(&token)
        .event_handler(cmd_handler)
        .await.expect("Error creating client");

    if let Err(why) = client.start().await{
        println!("Client error encountered: {:?}", why);
    }
}