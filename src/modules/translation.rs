use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    Args,
    macros::command,
    macros::group,
};

use crate::backend::translation::*;

const TRANSLATION_ERROR_MESSAGE: &str = "Someone spilled beans on the servers. Please try again in a bit!";

#[group]
#[description = "Various commands for translating and identifying the language of text"]
#[summary = "Translation commands"]
#[commands(detect)]
struct Translation;

#[command]
#[description = "Detects the language of the provided text"]
#[usage = "\"text\""]
#[example = "guten tag"]
#[min_args(1)]
pub async fn detect(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let trans_ctx = match data.get::<TranslationContextKey>(){
        None => {
            msg.channel_id.say(&ctx.http, &format!("Translation module is currently not enabled")).await?;
            return Ok(());
        },
        Some(tctx) => tctx
    };
    
    let target = match args.remains() {
        None => {
            msg.channel_id.say(&ctx.http, &format!("No text provided")).await?;
            return Ok(());
        },
        Some(text) => text
    };
    match detect_text(trans_ctx, target.to_string()).await {
        Err(_) => msg.channel_id.say(&ctx.http, TRANSLATION_ERROR_MESSAGE).await?,
        Ok(detection) => msg.channel_id.say(&ctx.http, &format!("I believe the language is `{:?}`!",detection.language)).await?
    };
    return Ok(());
}
