use std::{
    collections::HashSet,
    env,
    panic,
};

mod backend;
use backend::currency::*;
use backend::translation::*;

mod modules;
use modules::currency::*;
use modules::showmebeans::*;

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

struct CommandHandler;

#[async_trait]
impl EventHandler for CommandHandler{
    async fn ready(&self, ctx: Context, _data_about_bot: Ready){
        create_wallet_table();
        ctx.set_activity(Activity::listening("Quilla - Beans Beans Beans")).await;
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
            let _ = msg.channel_id.say(&ctx.http, &format!("You are missing the required role")).await;
        },
        DispatchError::OnlyForOwners => {
            let _ = msg.channel_id.say(&ctx.http, &format!("This command is only available to bot owners")).await;
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
    match create_token().await {
        Err(_) => println!("Failed to get token!"),
        Ok(token) => match translate("Testing, One, two, one, two".to_string(), token, Some("sv".to_string()), None).await {
            Err(_) => println!("Failed to translate!"),
            Ok(res) => println!("Result: {:?}, Lang: {:?}", res.translatedText, res.detectedSourceLanguage)
        }
    }
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file containing the bot token");
    } 
    let token = std::fs::read_to_string(std::path::Path::new(&args[1])).expect("Failed to open token file");

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
            .prefix("!")
            .case_insensitivity(true)
            .on_mention(Some(bot_id))
        )
        .on_dispatch_error(dispatch_error)
        .help(&MY_HELP)
        .group(&CURRENCY_GROUP)
        .group(&SHOWMEBEANS_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(CommandHandler)
        .await.expect("Error creating client");

    if let Err(why) = client.start().await{
        println!("Client error encountered: {:?}", why);
    }
}