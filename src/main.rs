use std::env;

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
    let token_result = std::fs::read_to_string(std::path::Path::new("../token"));
    assert_eq!(token_result.is_ok(), true);
    let token = token_result.ok().expect("Could not read token from file");

    let mut client = Client::builder(&token)
        .event_handler(CommandHandler)
        .await.expect("Error creating client");

    if let Err(why) = client.start().await{
        println!("Client error encountered: {:?}", why);
    }
}