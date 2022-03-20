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
use tokio_postgres::{NoTls, Error};


//Serenity General framework for commands
#[group]
#[commands(ping, test, uwu, help)]
struct General;

//Creating the message listener and handler and associated functions.
struct MessageHandler;

#[async_trait]
impl EventHandler for MessageHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("No token found");

    let db_connection_string = env::var("DB_CONNECTION")
        .expect("Database connection string not found, environment variable set?");

    let (db_client, db_connection) =
        tokio_postgres::connect(
            &db_connection_string
            ,NoTls)
            .await
            .expect("Connection Failed");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let mut discord_client = Client::builder(&token)
        .event_handler(MessageHandler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(error) = discord_client.start().await {
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

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "I am help").await?;

    Ok(())
}
