use serenity::prelude::*;
use serenity::{Result};
use serenity::model::prelude::*;
use serenity::framework::standard::{CommandResult, macros::*, Args};
use rand::random;
use std::collections::HashMap;

#[group]
#[commands(spinthebean)]
struct SpinTheBean;

#[command]
#[description = "Spins a bean to select a random user from the callers currents channel. Can alternatively be used with paramaters to choose between parameters"]
#[usage = "[list of items or blank for users in voice channel]"]
#[example("\"Black bean\", Edamame \"String Bean\"")]
#[only_in("guilds")]
#[delimiters(",", " ")]
#[aliases(stb, spin)]
pub async fn spinthebean(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() == 0{
        let voice_states: HashMap<UserId, VoiceState>;
        let voice_state: &VoiceState =  match msg.guild(&ctx.cache).await{
            None => {
                let _ = msg.channel_id.say(&ctx.http, "Could not fetch users in voice channel. No guild in cache").await;
                return Ok(())
            }
            Some(guild) => {
                voice_states = guild.voice_states;
                match voice_states.get(&msg.author.id){
                    Some(state) => state,
                    None => {
                        let _ = msg.channel_id.say(&ctx.http, "Join a voice channel or provide a list of items to spin").await;
                        return Ok(())
                    },
                }
            }
        };
        let channel: GuildChannel = match get_state_guild_channel(ctx, voice_state).await{
            Ok(chan) => chan,
            Err(_) => {
                let _ = msg.channel_id.say(&ctx.http, "Could not fetch users in voice channel").await;
                return Ok(())
            },
        };
        let members: Vec<Member> = match channel.members(&ctx.cache).await{
            Ok(mem) => {
                let mut filtered = Vec::new();
                for x in mem.iter(){
                    if !x.user.bot {
                        filtered.push(x.clone())
                    }
                }
                filtered
            },
            Err(_) => {
                let _ = msg.channel_id.say(&ctx.http, "Could not fetch users in voice channel").await;
                return Ok(())
            }
        };
        let result = members.get(random::<usize>() % members.len()).unwrap();
        let _ = msg.channel_id.say(&ctx.http, format!("The bean landed on *{}*!", result.display_name())).await;
    } else{
        let mut valid_args: Vec<String> = Vec::new();
        for _ in 0..args.len(){
            let _ = match args.single_quoted::<String>(){
                Ok(arg_string) => valid_args.push(arg_string),
                Err(_) => (),
            };
        }
        let rand_index: usize = random::<usize>() % valid_args.len();
        for x in 0..valid_args.len(){
            if rand_index == x{
                let _ = msg.channel_id.say(&ctx.http, format!("The bean landed on *{}*!", valid_args[x])).await;
                break;
            }
        }
    }
    Ok(())
}

async fn get_state_guild_channel(ctx: &Context, state: &VoiceState) -> Result<GuildChannel> {
    match state.channel_id{
        None => Err(SerenityError::Model(ModelError::ChannelNotFound)),
        Some(channel_id) => {
            match channel_id.to_channel_cached(&ctx.cache).await{
                None => Err(SerenityError::Model(ModelError::ChannelNotFound)),
                Some(channel) => {
                    match channel.guild(){
                        None => Err(SerenityError::Model(ModelError::ChannelNotFound)),
                        Some(gchannel) => Ok(gchannel),
                    }
                },
            }
        }
    }
}