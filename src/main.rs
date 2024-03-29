use std::{
    collections::HashSet,
    panic,
    io::Write
};

struct SettingsKey;
use ini::Ini;
impl TypeMapKey for SettingsKey {
    type Value = Ini;
}

use songbird::SerenityInit;

mod backend;
use backend::currency::*;
use backend::translation::*;
use backend::specialbeans::*;

mod modules;

use modules::currency::*;
use modules::showmebeans::*;
use modules::spinthebean::*;
use modules::beanverse::*;
use modules::translation::*;
use modules::specialbeans::*;
use modules::admin::*;
use modules::beanbird::*;

use serenity::{
    async_trait,
    framework::{
        StandardFramework,
        standard::Args,
        standard::HelpOptions,
        standard::CommandGroup,
        standard::CommandResult,
        standard::Configuration,
        standard::DispatchError,
        standard::help_commands,
        standard::macros::hook,
        standard::macros::help,
    },
    http::Http,
    model::{
        event::ResumedEvent,
        id::UserId,
        channel::Message,
        gateway::Ready,
        gateway::Activity,
        application::TeamMember,
    },
    prelude::*,
};

use crate::backend::markov::init_dirs;
use crate::backend::markov::init_chain_file;
use crate::backend::markov::init_chain_map;

struct CommandHandler;

#[async_trait]
impl EventHandler for CommandHandler{
    async fn ready(&self, ctx: Context, _data_about_bot: Ready){
        let settings = {
            let data = ctx.data.read().await;
            match data.get::<SettingsKey>() {
                None => panic!("How did the client even start without settings file?"),
                Some(set) => set.clone()
            }
        };
        create_spec_table();
        create_wallet_table();
        println!("Creating and/or loading markov chains");
        if let Err(text) = init_dirs().await {eprintln!("Failed to create dirs: {}", text)}
        if let Err(text) = init_chain_file("beanble", 3).await {eprintln!("Failed to create/load beanble: {}", text)}
        if let Err(text) = init_chain_map(&ctx).await {eprintln!("Failed to load chain maps: {}", text)}
        println!("Done!");
        initialize_translation(&ctx, &settings).await;
        ctx.set_activity(Activity::listening("Prefix: '?' We love beans!")).await;
        //ctx.set_activity(Activity::listening("Quilla - Beans Beans Beans")).await;
        println!("Hello! I am ready to dispatch beans!");
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::Ratelimited(info) => {
            if info.is_first_try {
                let _ = msg.channel_id.say(&ctx.http, &format!("Try this again in {} seconds.", info.as_secs())).await;
            }
        },
        DispatchError::LackingPermissions(info) => {
            let _ = msg.channel_id.say(&ctx.http, &format!("You are missing the following requirements {}", info)).await;
        },
        DispatchError::LackingRole => {
            let _ = msg.channel_id.say(&ctx.http, "You are missing the required role").await;
        },
        DispatchError::OnlyForOwners => {
            let _ = msg.channel_id.say(&ctx.http, "This command is only available to bot owners").await;
        },
        DispatchError::TooManyArguments{max,given} => {
            let _ = msg.channel_id.say(&ctx.http, &format!("Too many arguments! {:?} max, {:?} given",max,given)).await;
        },
        DispatchError::NotEnoughArguments{min,given} => {
            let _ = msg.channel_id.say(&ctx.http, &format!("Too few arguments! {:?} required, {:?} given",min,given)).await;
        },
        _ => ()
    }
}

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
#[tokio::main]
async fn main(){
    let set_path = std::path::Path::new("settings.ini");
    if !set_path.exists() {
        let mut setfile = std::fs::File::create(set_path).unwrap();
        let _ = setfile.write(b"discord_api_token=\n").unwrap();
        let _ = setfile.write(b"google_token_file=none\n").unwrap();
        panic!("No settings.ini file exists!\nCreating file...\nFill in settings.ini and relaunch the bot");
    }
    let setfile = match Ini::load_from_file(set_path) {
        Err(why) => panic!("Failed to load settings.ini! Error: {:?}", why),
        Ok(loaded) => loaded
    };

    let token = match setfile.general_section().get("discord_api_token") {
        None => panic!("Discord API token is not set in settings.ini"),
        Some(token) => token
    };

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.extend(team.members.iter().map(|m : &TeamMember| m.user.id));
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c: &mut Configuration| c
            .owners(owners)
            .prefix("?")
            .case_insensitivity(true)
            .on_mention(Some(bot_id))
        )
        .on_dispatch_error(dispatch_error)
        .help(&MY_HELP)
        .group(&CURRENCY_GROUP)
        .group(&SHOWMEBEANS_GROUP)
        .group(&TRANSLATION_GROUP)
        .group(&SPECIALBEANS_GROUP)
        .group(&BEANVERSE_GROUP)
        .group(&BEANBIRD_GROUP)
        .group(&BEANADMIN_GROUP)
        .group(&SPINTHEBEAN_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(CommandHandler)
        .register_songbird()
        .await.expect("Error creating client");

    client.data.write().await.insert::<SettingsKey>(setfile);

    if let Err(why) = client.start().await{
        println!("Client error encountered: {:?}", why);
    }
}