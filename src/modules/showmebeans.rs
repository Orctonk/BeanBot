use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    macros::group,
};

#[group]
#[commands(showmebeans)]
struct ShowMeBeans;

#[command]
pub async fn showmebeans(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|mut e| {
            e.title("***Beans***");
            e.color(Colour(16750123));
            e.image("https://cdn.onebauer.media/one/media/5dce/6968/90a4/1691/d1c0/c02c/Heinz+baked+beans.jpg?format=jpg&quality=80&width=960&height=540&ratio=16-9&resize=aspectfill");
            //return e
            e
        })
    }).await?;
    //Return Ok Result
    Ok(())
}