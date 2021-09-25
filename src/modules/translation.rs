use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    Args,
    macros::command,
    macros::group,
};
use std::str::FromStr;
use regex::Regex;
use chrono::Utc;

macro_rules! get_and_refresh_token {
    ($data:ident, $msg:ident, $ctx:ident) => {
        match $data.get::<TranslationContextKey>(){
            None => {
                $msg.channel_id.say(&$ctx.http, &format!("Translation module is currently not enabled")).await?;
                return Ok(());
            },
            Some(tctx) => {
                if tctx.token_expiry > Utc::now(){
                    tctx
                } else {
                    let rctx = match refresh_context(tctx.clone()).await {
                        Err(_) => {
                            $msg.channel_id.say(&$ctx.http, TRANSLATION_ERROR_MESSAGE).await?;
                            return Ok(());
                        },
                        Ok(rctx) => rctx
                    };
                    $data.insert::<TranslationContextKey>(rctx);
                    match $data.get::<TranslationContextKey>() {
                        None => {
                            $msg.channel_id.say(&$ctx.http, TRANSLATION_ERROR_MESSAGE).await?;
                            return Ok(());
                        },
                        Some(rctx) => rctx
                    }
                }
            }
        };
    };
}

use crate::backend::translation::*;

const TRANSLATION_ERROR_MESSAGE: &str = "Someone spilled beans on the servers. Please try again in a bit!";

struct OptionParseError;

struct TranslationOptions {
    source: Option<String>,
    target: String,
}

impl FromStr for TranslationOptions {
    type Err = OptionParseError;
    fn from_str(s: &str) -> Result<TranslationOptions,OptionParseError> {
        let re = Regex::new(r"\B\((\w+){0,1}->(\w+)\)\B").unwrap();
        let caps = match re.captures(s) {
            None => return Err(OptionParseError),
            Some(res) => res
        };

        let opt = TranslationOptions{
            source: match caps.get(1) {
                None => None,
                Some(mat) => Some(mat.as_str().to_string())
            },
            target: match caps.get(2){
                None => return Err(OptionParseError),
                Some(mat) => mat.as_str().to_string()
            }
        };
        Ok(opt)
    }
}

#[group]
#[description = "Various commands for translating and identifying the language of text"]
#[summary = "Translation commands"]
#[commands(detect,translate)]
struct Translation;

#[command]
#[description = "Translates of provided text"]
#[usage = "[([source]->target)] text"]
#[example = "(de->en) guten tag"]
#[min_args(1)]
pub async fn translate(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    let trans_ctx = get_and_refresh_token!(data,msg,ctx);

    let opts = match args.single::<TranslationOptions>() {
        Err(_) => TranslationOptions { source: None, target: "en".to_string()},
        Ok(opt) => opt
    };

    let target = match args.remains() {
        None => {
            msg.channel_id.say(&ctx.http, "No text provided").await?;
            return Ok(());
        },
        Some(text) => text
    };
    match translate_text(&trans_ctx, target.to_string(), Some(opts.target.clone()), opts.source.clone()).await {
        Err(TranslationError::ResponseError) => msg.channel_id.say(&ctx.http, "Invalid languages specified").await?,
        Err(_) => msg.channel_id.say(&ctx.http, TRANSLATION_ERROR_MESSAGE).await?,
        Ok(trans) => msg.channel_id.say(&ctx.http, &format!("Translated from `{:?}` to `{:?}`:```{:?}```",
            match opts.source { None => trans.detectedSourceLanguage.unwrap_or_else(|| "Unknown".to_string()), Some(lang) => lang},
            opts.target,
            trans.translatedText)
        ).await?
    };
    Ok(())
}

#[command]
#[description = "Detects the language of the provided text"]
#[usage = "\"text\""]
#[example = "guten tag"]
#[min_args(1)]
pub async fn detect(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    let trans_ctx = get_and_refresh_token!(data,msg,ctx);

    let target = match args.remains() {
        None => {
            msg.channel_id.say(&ctx.http, "No text provided").await?;
            return Ok(());
        },
        Some(text) => text
    };
    match detect_text(trans_ctx, target.to_string()).await {
        Err(_) => msg.channel_id.say(&ctx.http, TRANSLATION_ERROR_MESSAGE).await?,
        Ok(detection) => msg.channel_id.say(&ctx.http, &format!("I believe the language is `{:?}`!",detection.language)).await?
    };
    Ok(())
}
