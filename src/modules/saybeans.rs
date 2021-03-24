use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    macros::group,
};

#[group]
#[commands(saybeans)]
struct SayBeans;

#[command]
pub async fn saybeans(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "***Beans***").await?;

    Ok(())
}