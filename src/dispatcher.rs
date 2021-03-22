use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateMessage},
    model::{channel::Message, gateway::Ready, id::ChannelId, gateway::Activity},
    prelude::*,
    utils::Colour,
};
pub struct Dispatcher;

impl Dispatcher{
    pub async fn dispatch(&self, ctx: Context, new_message: &Message, parsed_message: &String){
        match new_message.channel_id.send_message(&ctx.http, |m: &mut CreateMessage| 
                m.embed(|e: &mut CreateEmbed| e.colour(Colour::new(16744210)).title("***Beans.***").image("https://cdn.onebauer.media/one/media/5dce/6968/90a4/1691/d1c0/c02c/Heinz+baked+beans.jpg?format=jpg&quality=80&width=960&height=540&ratio=16-9&resize=aspectfill"))
            ).await{
            Err(why) => println!("Error sending message: {:?}", why),
            Ok(_) => println!("Dispatched beans.... with message {:?}", parsed_message)
        }
    }
}

