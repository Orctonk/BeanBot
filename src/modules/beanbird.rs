use std::{
    sync::Arc,
    time::Duration
};
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::{
    async_trait,
    builder::CreateEmbed,
    http::Http,
    framework::standard::{CommandResult, macros::command, macros::group, Args, Delimiter},
    Result as SerenityResult,

};
use songbird::{input::{
    restartable::Restartable,
    Metadata,
    Input
}, EventContext,
   Event,
   EventHandler as VoiceEventHandler,
   TrackEvent,
   Songbird
};

#[group]
#[description = "Play youtube audio"]
#[summary = "Plays youtube audio using youtube-dl"]
#[commands(join, leave, stop, queue, skip, pause, resume)]
struct BeanBird;

//Struct for implementing actions on a track play event
struct TrackPlayNotifier {
    chan_id: ChannelId,
    http: Arc<Http>,
}

/**
Gets guild id and songbird-manager from a given context and message
*/
async fn get_voice_info(ctx: &&Context, msg: &Message) -> (GuildId, Arc<Songbird>) {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.");
    (guild_id, manager)
}

#[command]
#[only_in(guilds)]
#[description = "Stops any currently playing song and removes it from the queue"]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let (guild_id, manager) = get_voice_info(&ctx, msg).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        if handler.queue().pause().is_ok() && handler.queue().dequeue(0).is_some() {
            check_msg(msg.channel_id.say(&ctx.http, "Stopped playing").await);
        }
        else {
            check_msg(msg.channel_id.say(&ctx.http, "Error pausing and dequeuing").await);
        }
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description = "Pauses the currently playing song"]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let (guild_id, manager) = get_voice_info(&ctx, msg).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        if handler.queue().pause().is_ok() {
            check_msg(msg.channel_id.say(&ctx.http, "Paused current song").await);
        } else {
            check_msg(msg.channel_id.say(&ctx.http, "Error pausing song").await);
        }


    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description = "Resumes playing the song at the front of the queue"]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    let (guild_id, manager) = get_voice_info(&ctx, msg).await;

    match manager.get(guild_id) {
        Some(handler_lock) => {
            let handler = handler_lock.lock().await;
            if handler.queue().resume().is_err(){
                check_msg(msg.channel_id.say(&ctx.http, "Error unpausing song").await);
            }
        }
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await);
        }
    }

    Ok(())
}

#[async_trait]
impl VoiceEventHandler for TrackPlayNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        //If event is track event (only event added is for track starts)
        if let EventContext::Track(track_list) = ctx {
            //Extract handle and create embed using new tracks metadata
            if let Some((_, handle)) = track_list.get(0) {
                check_msg(self.chan_id
                    .send_message(
                        &self.http,
                        |m| {
                            m.embed(|e| {
                                create_song_embed(
                                    e,
                                    handle.metadata(),
                                    1,
                                    "Playing song".to_string()
                                )
                            })
                        })
                    .await);
            }
        }
        None
    }
}

#[command]
#[only_in(guilds)]
#[max_args(1)]
#[description = "Summons the bot to your current channel or to a specified channel"]
async fn join(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let connect_to: ChannelId;

    //If no args are given fetch users channel
    if args.is_empty() {
        let channel_id = guild
            .voice_states
            .get(&msg.author.id)
            .and_then(|voice_state| voice_state.channel_id);

        connect_to = match channel_id {
            Some(channel) => channel,
            None => {
                check_msg(msg.reply(ctx, "Not in a voice channel").await);

                return Ok(());
            },
        };
    } else { //Else check args for channel id
        let input_channel = args.single::<String>().unwrap();
        let id = &input_channel[2..input_channel.len()-1];
        match id.parse::<u64>() {
            Ok(parsed_id) => {connect_to = ChannelId::from(parsed_id)}
            Err(_) => {
                check_msg(msg.reply(ctx, "Invalid channel given").await);
                return Ok(());
            }
        }
        //Test if valid
        if connect_to.to_channel(&ctx.http).await.is_err() {
            check_msg(msg.reply(ctx, "Invalid channel given").await);
            return Ok(());
        }
    }

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.");

    //Join channel
    let (handle_lock, success) = manager.join(guild_id, connect_to).await;

    if let Ok(_channel) = success {
        check_msg(
            msg.channel_id
                .say(&ctx.http, &format!("Joined {}", connect_to.mention()))
                .await,
        );

        let chan_id = msg.channel_id;

        //Clone http to store in event handler
        let send_http = ctx.http.clone();

        let mut handle = handle_lock.lock().await;

        //Add "track play" event to handle
        handle.add_global_event(
            Event::Track(TrackEvent::Play),
            TrackPlayNotifier {
                chan_id,
                http: send_http,
            },
        );

    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Error joining the channel")
                .await,
        );
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description = "Makes the bot leave its current channel"]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let (guild_id, manager) = get_voice_info(&ctx, msg).await;
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel").await);
    } else {
        check_msg(msg.reply(ctx, "Not in a voice channel").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases(play)]
#[min_args(1)]
#[description = "Queues a song to be played. Immediately starts playing if no songs are in queue"]
async fn queue(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    //Check for args
    let url = String::from(args.rest());
    if url.is_empty() {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Must provide a URL or a search term")
                .await,
        );
        return Ok(());
    };

    let (guild_id, manager) = get_voice_info(&ctx, msg).await;

    if manager.get(guild_id).is_none() {
        let _ = join(
            ctx,
            msg,
            Args::new("", &[Delimiter::Single(' ')])
        ).await;
    }

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match if url.starts_with("https://") { Restartable::ytdl(url.clone(), true).await }
        else { Restartable::ytdl_search(url.clone(), true).await } {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error starting source (might be age restricted").await);

                return Ok(());
            },
        };

        let input: Input = source.into();
        let metadata = input.metadata.clone();

        handler.enqueue_source(input);
        check_msg(msg.channel_id
            .send_message(
                &ctx.http,
                |m| {
                    m.embed(|e| {
                        create_song_embed(
                            e,
                            &metadata,
                            handler.queue().len(),
                            if handler.queue().len() == 1 {"Playing song".to_string()} else { "Queued song".to_string() }
                        )
                    })
                })
            .await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}

/**
Creates a formatted embed from given metadata, queue length, and embed title. Takes a CreateEmbed
builder used when creating embeds.
*/
fn create_song_embed<'a, 'b>(e: &'a mut CreateEmbed, metadata: &'b Metadata, queue_len: usize, author: String) -> &'a mut CreateEmbed {
    e.url(metadata.source_url.clone().unwrap_or_else(|| "No url".to_string()))
        .author(|a| {
            a.name(author)
        });
    let title = if metadata.source_url.clone().is_some() {
        format!("[{}]({})", metadata.title.as_ref().unwrap_or(&"No title".to_string()), metadata.source_url.clone().unwrap())
    } else { metadata.title.as_ref().unwrap_or(&"No title".to_string()).to_string() };

    e.field("Title".to_string(), title, true)
        .thumbnail(metadata.thumbnail.as_ref().unwrap_or(&"".to_string()))
        .field("Position in queue".to_string(), format!("{}", queue_len) , true);
    let duration = metadata.duration.unwrap_or_else(|| Duration::from_secs(0));
    let hours = duration.as_secs() / 3600;
    let minutes = duration.as_secs() % 3600 / 60;
    let seconds = duration.as_secs() % 60;

    e.field("Length".to_string(), format!("{}:{}:{}", time_to_text(hours), time_to_text(minutes), time_to_text(seconds)), true)
}

/**
Formats a time (in seconds) to text with the format: hh:mm:ss
*/
fn time_to_text(time: u64) -> String {
    let mut res = time.to_string();
    if time < 10 {
        res.insert(0, '0');
    }
    res
}

#[command]
#[only_in(guilds)]
#[description = "Skips the currently playing song"]
async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let (guild_id, manager) = get_voice_info(&ctx, msg).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();
        if queue.len() == 1 {
            check_msg(
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.author(|a| a.name("End of queue".to_string()))
                        })
                    }).await
            );
        }
        
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "I am not currently in a voice channel")
                .await,
        );
    }

    Ok(())
}

/**
Prints message error to terminal if error occurred in given SerenityResult
*/
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}