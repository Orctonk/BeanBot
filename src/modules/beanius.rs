use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::model::prelude::*;
use serenity::model::id::UserId;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    macros::group,
};
use super::super::Interactive;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct ActiveGame{
    pub message_reference: Option<serenity::model::channel::MessageReference>,
    pub stage: GameStage
}

impl Clone for ActiveGame {
    fn clone(&self) -> Self {
        ActiveGame {
            message_reference: self.message_reference.clone(),
            stage: self.stage
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameStage{
    PickCategory,
    PickQuestion,
    PlayAgain
}

pub struct ActiveGames;

//Nyckeltyp för SharedMap
impl TypeMapKey for ActiveGames{
    //UserID and game stage
    type Value = Arc<RwLock<HashMap<UserId, ActiveGame>>>;
}

pub async fn init_beanius(ctx: &Context){
    let mut data = ctx.data.write().await;
    data.insert::<ActiveGames>(Arc::new(RwLock::new(HashMap::default())));
}


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
    let map_lock = {
        let data = ctx.data.read().await;
        data.get::<ActiveGames>().expect("Beanius error: Unitiated hash map").clone()
    };

    let game= {
        let mut active_games = map_lock.write().await;

        active_games.entry(msg.author.id).or_insert(ActiveGame{
            message_reference: msg.message_reference.clone(),
            stage: GameStage::PickCategory
        }).clone()
    };
    
    println!("{:?}", game);
    
    match game.stage {
        GameStage::PickCategory => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.content("Pick a category!");
                m.components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|cb| {
                            cb.style(ButtonStyle::Primary);
                            cb.custom_id::<String>(String::from("Science"));
                            cb.label("Science");
                            cb
                        });
                        ar.create_button(|cb| {
                            cb.style(ButtonStyle::Primary);
                            cb.custom_id::<String>(String::from("Sports"));
                            cb.label("Sports");
                            cb
                        });
                        ar.create_button(|cb| {
                            cb.style(ButtonStyle::Primary);
                            cb.custom_id::<String>(String::from("History"));
                            cb.label("History");
                            cb
                        });
                        ar.create_button(|cb| {
                            cb.style(ButtonStyle::Primary);
                            cb.custom_id::<String>(String::from("Popular Culture"));
                            cb.label("Popular Culture");
                            cb
                        })
                    })
                })
            }).await.expect("Could not send message");
        },
        _ => {msg.reply(&ctx.http, "You already have a running game").await.expect("Could not reply");}
    }
    Ok(())
}