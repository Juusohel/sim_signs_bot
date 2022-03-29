#![allow(unused_imports)]
#![allow(dead_code)]

use std::{env, fs};
use std::sync::Arc;

use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::futures::future::err;
use serenity::model::connection::Connection;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use tokio_postgres::tls::NoTlsStream;
use tokio_postgres::{Error, NoTls, Socket};

//Setting up container for the psql client
struct ZodiacClient {
    tokio_postgres: tokio_postgres::Client,
}

impl TypeMapKey for ZodiacClient {
    type Value = Arc<tokio_postgres::Client>;
}

// Serenity General framework for commands
#[group]
#[commands(uwu, help, sign, setsign, car, track, compatibility, monthly)]
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
    tokio::spawn(async move {
        if let Err(e) = db_connection.await {
            eprintln!("connection error: {}", e);
        }
    });

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

// Uses the psql client and the user_id to retrieve the stored zodiac sign from the database
#[command]
async fn sign(ctx: &Context, msg: &Message) -> CommandResult {
    // Pulling in psql client
    let read = ctx.data.read().await;
    let client = read
        .get::<ZodiacClient>()
        .expect("PSQL client error")
        .clone();
    let user_id = msg.author.id.as_u64().to_string(); // User_ID to be used as the key

    // Querying database for the stored sign
    let rows = client
        .query(
            "SELECT user_zodiac_sign FROM user_sign WHERE user_id = $1",
            &[&user_id],
        )
        .await
        .expect("Error querying the database, database set correctly?");

    // If result is not empty, display the stored sign, if empty instruct the user to set it
    if rows.len() > 0 {
        let value: &str = rows[0].get(0);
        let reply_string = format!("<@{}> Your sign is {}!", msg.author.id, value);
        msg.reply(ctx, reply_string).await?;
    } else {
        let no_sign_set_string = format!(
            "<@{}> Your sign is not set! Set your sign with ~set <Sign>",
            msg.author.id
        );
        msg.reply(ctx, no_sign_set_string).await?;
    }

    Ok(())
}

// Command that allows a user to set their zodiac sign
// Parses a parameter from the message and saves it to the database if it's considered a valid sign.
#[command]
async fn setsign(ctx: &Context, msg: &Message) -> CommandResult {
    // Pulling in psql client
    let read = ctx.data.read().await;
    let client = read
        .get::<ZodiacClient>()
        .expect("PSQL client error")
        .clone();
    let mut valid: bool = true;
    let user_id = msg.author.id.as_u64().to_string(); // User_ID to be used as the key
    let mut user_new_sign = String::from("default");
    let user_message = &msg.content.to_lowercase()[9..]; //parsing sign from message

    // Matches the command to the desired zodiac sign and assigns it to the variable
    match user_message {
        "aries" => user_new_sign = String::from("aries"),
        "taurus" => user_new_sign = String::from("taurus"),
        "gemini" => user_new_sign = String::from("gemini"),
        "cancer" => user_new_sign = String::from("cancer"),
        "leo" => user_new_sign = String::from("leo"),
        "virgo" => user_new_sign = String::from("virgo"),
        "libra" => user_new_sign = String::from("libra"),
        "scorpio" => user_new_sign = String::from("scorpio"),
        "sagittarius" => user_new_sign = String::from("sagittarius"),
        "capricorn" => user_new_sign = String::from("capricorn"),
        "aquarius" => user_new_sign = String::from("aquarius"),
        "pisces" => user_new_sign = String::from("pisces"),
        _ => {
            let reply = format!(
                "{} is not a valid sign! Please enter a valid sign.",
                user_message
            );
            valid = false;
            msg.reply(ctx, reply).await?;
        }
    }

    // If the command parameter is considered a acceptable zodiac sign it is saved to the database.
    // If the user has a previously saved zodiac sign, the new sign gets updated in its place.
    if valid {
        let _statement = client
            .execute(
                "INSERT INTO user_sign (user_id, user_zodiac_sign)
            VALUES ($1, $2)
            ON CONFLICT (user_id)
            DO
            UPDATE SET user_zodiac_sign = EXCLUDED.user_zodiac_sign",
                &[&user_id, &user_new_sign],
            )
            .await
            .expect("broken insert");
        let reply = format!("<@{}> Your zodiac sign has been set!", msg.author.id);
        msg.channel_id.say(ctx, reply).await?;
    }

    Ok(())
}

#[command]
async fn car(ctx: &Context, msg: &Message) -> CommandResult {
    // Pulling in psql client
    let read = ctx.data.read().await;
    let client = read
        .get::<ZodiacClient>()
        .expect("PSQL client error")
        .clone();
    let user_id = msg.author.id.as_u64().to_string(); // User_ID to be used as the key

    // Querying database for the stored sign
    let rows = client
        .query(
            "SELECT user_zodiac_sign FROM user_sign WHERE user_id = $1",
            &[&user_id],
        )
        .await
        .expect("Error querying the database, database set correctly?");

    // If result is not empty, take the stored sign and pick a file based on it.
    if rows.len() > 0 {
        let value: &str = rows[0].get(0);
        let filepath = format!("content/cars/{}.txt",value);
        let zodiac_contents = fs::read_to_string(filepath)?;

        msg.channel_id.say(ctx, zodiac_contents).await?;
    } else {
        let no_sign_set_string = format!(
            "<@{}> Your sign is not set! Set your sign with ~set <Sign>",
            msg.author.id
        );
        msg.reply(ctx, no_sign_set_string).await?;
    }

    Ok(())
}

#[command]
async fn track(ctx: &Context, msg: &Message) -> CommandResult {
    // Pulling in psql client
    let read = ctx.data.read().await;
    let client = read
        .get::<ZodiacClient>()
        .expect("PSQL client error")
        .clone();
    let user_id = msg.author.id.as_u64().to_string(); // User_ID to be used as the key

    // Querying database for the stored sign
    let rows = client
        .query(
            "SELECT user_zodiac_sign FROM user_sign WHERE user_id = $1",
            &[&user_id],
        )
        .await
        .expect("Error querying the database, database set correctly?");

    // If result is not empty, take the stored sign and pick a file based on it.
    if rows.len() > 0 {
        let value: &str = rows[0].get(0);
        let filepath = format!("content/tracks/{}.txt",value);
        let zodiac_contents = fs::read_to_string(filepath)?;

        msg.channel_id.say(ctx, zodiac_contents).await?;
    } else {
        let no_sign_set_string = format!(
            "<@{}> Your sign is not set! Set your sign with ~set <Sign>",
            msg.author.id
        );
        msg.reply(ctx, no_sign_set_string).await?;
    }


    Ok(())
}

#[command]
async fn compatibility(ctx: &Context, msg: &Message) -> CommandResult {
    // Parse user from the command text
    // Check the 2 zodiacs and select a compatibility text based on the pairing
    // if running out of time, determine level of compatibility 1-10 and select text based on that
    // texts have to be more generic though

    Ok(())
}

#[command]
async fn monthly(ctx: &Context, msg: &Message) -> CommandResult {
    // Pulling in psql client
    let read = ctx.data.read().await;
    let client = read
        .get::<ZodiacClient>()
        .expect("PSQL client error")
        .clone();
    let user_id = msg.author.id.as_u64().to_string(); // User_ID to be used as the key

    // Querying database for the stored sign
    let rows = client
        .query(
            "SELECT user_zodiac_sign FROM user_sign WHERE user_id = $1",
            &[&user_id],
        )
        .await
        .expect("Error querying the database, database set correctly?");

    // If result is not empty, take the stored sign and pick a file based on it.
    if rows.len() > 0 {
        let value: &str = rows[0].get(0);
        let filepath = format!("content/monthly/{}.txt",value);
        let zodiac_contents = fs::read_to_string(filepath)?;

        msg.channel_id.say(ctx, zodiac_contents).await?;
    } else {
        let no_sign_set_string = format!(
            "<@{}> Your sign is not set! Set your sign with ~set <Sign>",
            msg.author.id
        );
        msg.reply(ctx, no_sign_set_string).await?;
    }


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
    // ADD PROPER HELP LATER
    Ok(())
}
