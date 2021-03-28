use serenity::prelude::*;
use serenity::Result;
use serenity::model::prelude::*;
use serenity::framework::standard::{CommandResult, macros::*, Args};
use rand::random;
use std::collections::HashMap;

#[group]
#[commands(spinthebean)]
struct SpinTheBean;

#[command]
#[description = "Spins a bean to select a random user from the callers currents channel. Can alternatively be used with paramaters to choose between parameters"]
#[delimiters(",", " ")]
#[aliases(stb)]
pub async fn spinthebean(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() == 0{
        let voice_states: HashMap<UserId, VoiceState>;
        let voice_state: &VoiceState =  match msg.guild(&ctx.cache).await{
            None => {
                msg.channel_id.say(&ctx.http, "Could not fetch users in voice channel. No guild in cache").await;
                return Ok(())
            }
            Some(guild) => {
                voice_states = guild.voice_states;
                match voice_states.get(&msg.author.id){
                    Some(state) => {
                        state
                    },
                    None => {
                        msg.channel_id.say(&ctx.http, "Could not fetch users in voice channel. You are not in a voice channel").await;
                        return Ok(())
                    },
                }
            }
        };
        let channel: GuildChannel = match get_state_guild_channel(ctx, voice_state).await{
            Ok(chan) => {
                chan
            },
            Err(_) => {
                msg.channel_id.say(&ctx.http, "Could not fetch users in voice channel").await;
                return Ok(())
            },
        };
        let members: Vec<Member> = match channel.members(&ctx.cache).await{
            Ok(mem) => mem,
            Err(_) => {
                msg.channel_id.say(&ctx.http, "Could not fetch users in voice channel").await;
                return Ok(())
            }
        };
        let rand_index: usize = random::<usize>() % members.len();
        let result = members.get(rand_index).unwrap();
        msg.channel_id.say(&ctx.http, format!("The bean landed on {}!", result.display_name())).await;
    } else{
        let mut valid_args: Vec<String> = Vec::new();
        let rand_index: usize = random::<usize>() % args.len();
        for _ in 0..args.len(){
            let _ = match args.single_quoted::<String>(){
                Ok(arg_string) => valid_args.push(arg_string),
                Err(_) => (),
            };
        }
        for x in 0..valid_args.len(){
            if rand_index == x{
                msg.channel_id.say(&ctx.http, format!("The bean landed on {}!", valid_args[x])).await;
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