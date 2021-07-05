use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    macros::group,
};

#[group]
#[description = "Commands for playing beanius."]
#[summary = "Play beanius."]
#[commands(beanius)]
struct Beanius;

#[command]
#[description = "Initiates a game of beanius"]
#[max_args(2)]
#[usage = "@User [bet]"]
pub async fn beanius(ctx: &Context, msg: &Message) -> CommandResult<()>{
    if let Err(e) = msg.channel_id.send_message(&ctx.http, |m| {
        m.content("This is Totally Real!");
        m.components(|c| {
            c.create_action_row(|ar| ar.create_button(|cb| {
                cb.style(ButtonStyle::Primary);
                cb.custom_id::<u8>(0);
                cb.label("Sport");
                cb
            }));
            c
        });
        m
    }).await {
        eprintln!("{}", e);
    }
    Ok(())
}