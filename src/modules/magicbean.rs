use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{CommandResult, macros::command, macros::group, CommandError};



#[group]
#[commands(magicbean)]
#[description = "A group with commands related magicbean"]
#[summary = "The magic bean knows the answer"]
struct MagicBean;

#[command]
#[description = "Magic bean answers your question"]
#[usage = ""]
#[example = ""]
pub async fn magicbean(ctx: &Context, msg: &Message) -> CommandResult {
    match msg.channel_id.send_message(&ctx.http, |m| {
        m.content("The magic bean knows but won't tell you.")
    }).await {
        Ok(_) => Ok(()),
        Err(e) => Err(CommandError::from(e))
    }
}