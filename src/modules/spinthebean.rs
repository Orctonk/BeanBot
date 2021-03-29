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
        let mut names: Vec<String> = Vec::new();
        let members: Vec<Member> = match channel.members(&ctx.cache).await{
            Ok(mem) => {
                let mut filtered = Vec::new();
                for x in mem.iter(){
                    if !x.user.bot {
                        filtered.push(x.clone());
                        names.push(match &x.nick {
                            Some(nick) => nick.to_string(),
                            _ => x.user.name.to_string()
                        })
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
        let _ = msg.channel_id.send_message(&ctx.http, |m|{
            m.embed(|e| {
                e.author(|f|
                    f.name("Kanna Beans")
                        .icon_url("https://cdn.discordapp.com/avatars/354361968091594752/3cd9f38df78c761bd5b059797cbd6fec.png?size=128"))
                    .title(format!("The bean landed on __{}__", match &result.nick {
                        None => {result.user.name.to_string()}
                        Some(nick) => {nick.to_string()}
                    }))
                    .color(16750123)
                    .thumbnail("https://media1.tenor.com/images/f5b2182314ec6603e4015cb03497bdf9/tenor.gif?itemid=10565478");
                for (i, mem) in members.iter().enumerate(){
                    e.field(&names[i], if mem.user.eq(&result.user) {":point_up:"} else {"_"}, true);
                }
                e
            })}).await;
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
                eprintln!("{}", "Sending embed");
                let _ = msg.channel_id.send_message(&ctx.http, |m|{
                    m.embed(|e| {
                            e.author(|f|
                                f.name("Kanna Beans")
                                    .icon_url("https://cdn.discordapp.com/avatars/354361968091594752/3cd9f38df78c761bd5b059797cbd6fec.png?size=128"))
                            .title(format!("The bean landed on __{}__", valid_args[x]))
                            .color(16750123)
                            .thumbnail("https://media1.tenor.com/images/f5b2182314ec6603e4015cb03497bdf9/tenor.gif?itemid=10565478");
                        for (i, arg) in valid_args.iter().enumerate(){
                            e.field(arg, if i == rand_index {":point_up:"} else {"_"}, true);
                        }
                        e
                    })}).await;
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