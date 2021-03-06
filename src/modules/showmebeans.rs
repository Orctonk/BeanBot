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
#[description = "A group with commands related to showing pictures of beans"]
#[summary = "Show beans"]
struct ShowMeBeans;

#[command]
#[description = "Shows a picture of beans"]
#[usage = ""]
#[example = ""]
pub async fn showmebeans(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("***Beans***");
            e.color(Colour(16750123));
            e.image("https://cdn.onebauer.media/one/media/5dce/6968/90a4/1691/d1c0/c02c/Heinz+baked+beans.jpg?format=jpg&quality=80&width=960&height=540&ratio=16-9&resize=aspectfill");
            e
        })
    }).await?;
    //Return Ok Result
    Ok(())
}