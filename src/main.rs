use std::env;
use std::sync::Arc;

use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::futures::future::err;
use tokio_postgres::{Error, NoTls};

//Setting up container for the psql client
struct ZodiacClient {
    tokio_postgres: tokio_postgres::Client
}

impl TypeMapKey for ZodiacClient {
    type Value = Arc<tokio_postgres::Client>;
}

// Serenity General framework for commands
#[group]
#[commands(ping, test, uwu, help, whatismysign)]
struct General;

// Creating the message handler and associated functions.
struct MessageHandler;

#[async_trait]
impl EventHandler for MessageHandler {
    // Ready gets dispatched by default when the bot starts up, overriding to confirm in CL
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Setting discord token for the bot to use
    let token = env::var("DISCORD_TOKEN").expect("No token found");

    // Setting database connection string to be used by the database client
    // Format: host= <> dbname= <> user= <> password= <>
    let db_connection_string = env::var("DB_CONNECTION")
        .expect("Database connection string not found, environment variable set?");

    // Connecting to the PostgreSQL database.
    let (db_client, db_connection) = tokio_postgres::connect(&db_connection_string, NoTls)
        .await
        .expect("Connection Failed");

    // Moving the actual connection object to its own thread
//    tokio::spawn(async move {
//        if let Err(e) = db_connection.await {
//            eprintln!("connection error: {}", e);
//        }
//    });

    // Creating serenity bot framework and its configuration
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    // Building serenity discord bot client using the auth token, framework and message handler defined above
    let mut discord_client = Client::builder(&token)
        .event_handler(MessageHandler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // Creating psql client container
    {
        let mut data = discord_client.data.write().await;
        data.insert::<ZodiacClient>(Arc::new(db_client));
    }

    // Starting client
    if let Err(error) = discord_client.start().await {
        println!("Client error {:?}", error)
    }
}

// Function to get the thing from database
//async fn get_user_sign(user_id: UserId, db_client: tokio_postgres::Client) {
//    let user_id = user_id.as_u64().to_string();
//    let rows = db_client
//        .query(
//            "SELECT zodiac_sign FROM user_signs WHERE user_id = $1",
//            &[&user_id],
//        )
//        .await;
//    // actually return it
//}

#[command]
async fn whatismysign(ctx: &Context, msg: &Message) -> CommandResult {
    let read = ctx.data.read().await;
    println!("grabbing client"); //debug
    let client = read.get::<ZodiacClient>().expect("PSQL client error").clone();

    let user_id = msg.author.id.as_u64().to_string();
    println!("got to the bit before query"); //debug
    let rows = client
        .query( //hardcoded query for testing
            "SELECT * FROM user_signs WHERE user_zodiac_sign=$1",
            &[&"aries"],
        )
        .await
        .expect("query error");
    println!("finished query"); //debug


    Ok(())


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

