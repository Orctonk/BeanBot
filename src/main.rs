use std::env;
use std::panic;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct CommandHandler;

#[async_trait]
impl EventHandler for CommandHandler{
    async fn message(&self, ctx: Context, new_message: Message){
        
    }

    async fn ready(&self, _: Context, data_about_bot: Ready){
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

    let mut client = Client::builder(&token)
        .event_handler(CommandHandler)
        .await.expect("Error creating client");

    if let Err(why) = client.start().await{
        println!("Client error encountered: {:?}", why);
    }
}