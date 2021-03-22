use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateMessage},
    model::{channel::Message, gateway::Ready, id::ChannelId, gateway::Activity},
    prelude::*,
    utils::Colour,
};

use crate::module::Module;

pub struct Dispatcher;

impl Dispatcher{
    pub async fn dispatch(&self, ctx: Context, new_message: &Message, parsed_message: &String){
        // find module based on command and dispatch to it
        match new_message.channel_id.send_message(&ctx.http, |m: &mut CreateMessage| m.content("Beans")).await{
            Err(why) => println!("Error sending message: {:?}", why),
            Ok(_) => println!("Dispatched beans....")
        }
    }

    pub async fn addModule(&self, module: &Module){
        // Add model commands to hashmap with module as value
    }
}

