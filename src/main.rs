use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};


const HELP_MSG: &str = "Hello I am help";
const HELP_CMD: &str = "!help";

#[group]
#[commands(ping, test, uwu)]
struct General;

struct MessageHandler;

#[async_trait]
impl EventHandler for MessageHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_CMD {
            if let Err(error) = msg.channel_id.say(&ctx.http, HELP_MSG).await {
                println!("Sending message failed: {:?}", error);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("No token found");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);



    let mut client = Client::builder(&token)
        .event_handler(MessageHandler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(error) = client.start().await {
        println!("Client error {:?}", error)
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn test(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Hello test reply").await?;

    Ok(())
}

#[command]
async fn uwu(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "UwU").await?;

    Ok(())
}