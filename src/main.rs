use std::sync::Arc;
use std::{env, fs};

use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};

use serenity::utils::Color;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use tokio_postgres::NoTls;

//Setting up container for the psql client
struct ZodiacClient {
    _tokio_postgres: tokio_postgres::Client,
}

impl TypeMapKey for ZodiacClient {
    type Value = Arc<tokio_postgres::Client>;
}

// Serenity General framework for commands
#[group]
#[commands(uwu, help, sign, set, car, track, monthly, deleteme)]
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
        .configure(|c| c.prefix("~").case_insensitivity(true))
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

// Finds
async fn find_image_url(sign: &str) -> String {
    let image_url;
    match sign {
        "aries" => image_url = String::from("https://i.imgur.com/dAuAGgd.png"),
        "taurus" => image_url = String::from("https://i.imgur.com/qK6mLmC.png"),
        "gemini" => image_url = String::from("https://i.imgur.com/B3SMMZx.png"),
        "cancer" => image_url = String::from("https://i.imgur.com/Xwcnwdp.png"),
        "leo" => image_url = String::from("https://i.imgur.com/SylDKH4.png"),
        "virgo" => image_url = String::from("https://i.imgur.com/43fQB8O.png"),
        "libra" => image_url = String::from("https://i.imgur.com/fd0c1D6.png"),
        "scorpio" => image_url = String::from("https://i.imgur.com/96Ggy0a.png"),
        "sagittarius" => image_url = String::from("https://i.imgur.com/28gC6XX.png"),
        "capricorn" => image_url = String::from("https://i.imgur.com/fULgjr3.png"),
        "aquarius" => image_url = String::from("https://i.imgur.com/6WhFiEf.png"),
        "pisces" => image_url = String::from("https://i.imgur.com/wBzGPSf.png"),
        _ => image_url = String::from("https://cdn.discordapp.com/attachments/729394298860208198/958502362526384128/unknown.png") // Should be unreachable as invalid signs not allowed in database

    }
    image_url
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
async fn set(ctx: &Context, msg: &Message) -> CommandResult {
    // Pulling in psql client
    let read = ctx.data.read().await;
    let client = read
        .get::<ZodiacClient>()
        .expect("PSQL client error")
        .clone();
    let mut valid: bool = true;
    let user_id = msg.author.id.as_u64().to_string(); // User_ID to be used as the key
    let mut user_new_sign = String::from("default");
    let user_message = &msg.content.to_lowercase()[5..]; //parsing sign from message

    // Matches the command to the desired zodiac sign and assigns it to the variable
    match user_message {
        "aries" => user_new_sign = String::from("Aries"),
        "taurus" => user_new_sign = String::from("Taurus"),
        "gemini" => user_new_sign = String::from("Gemini"),
        "cancer" => user_new_sign = String::from("Cancer"),
        "leo" => user_new_sign = String::from("Leo"),
        "virgo" => user_new_sign = String::from("Virgo"),
        "libra" => user_new_sign = String::from("Libra"),
        "scorpio" => user_new_sign = String::from("Scorpio"),
        "sagittarius" => user_new_sign = String::from("Sagittarius"),
        "capricorn" => user_new_sign = String::from("Capricorn"),
        "aquarius" => user_new_sign = String::from("Aquarius"),
        "pisces" => user_new_sign = String::from("Pisces"),
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
        let filepath = format!("content/cars/{}.txt", value.to_lowercase());
        let zodiac_contents = fs::read_to_string(filepath)?;
        let image_url = find_image_url(&value.to_lowercase()).await;

        msg.channel_id
            .send_message(ctx, |message| {
                message
                    .content(format!(
                        "<@{}> the stars have spoken and you should choose the following car!",
                        msg.author.id
                    ))
                    .embed(|embed| {
                        embed
                            .title(value)
                            .description(zodiac_contents)
                            .color(Color::DARK_BLUE)
                            .image(image_url)
                    })
            })
            .await?;
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
        let filepath = format!("content/tracks/{}.txt", value.to_lowercase());
        let zodiac_contents = fs::read_to_string(filepath)?;
        let image_url = find_image_url(&value.to_lowercase()).await;

        msg.channel_id
            .send_message(ctx, |message| {
                message
                    .content(format!(
                        "<@{}> the stars have spoken and you should race on the following track!",
                        msg.author.id
                    ))
                    .embed(|embed| {
                        embed
                            .title(value)
                            .description(zodiac_contents)
                            .color(Color::DARK_BLUE)
                            .image(image_url)
                    })
            })
            .await?;
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
        let filepath = format!("content/monthly/{}.txt", value.to_lowercase());
        let zodiac_contents = fs::read_to_string(filepath)?;
        let image_url = find_image_url(&value.to_lowercase()).await;

        msg.channel_id
            .send_message(ctx, |message| {
                message
                    .content(format!(
                        "<@{}> the stars have spoken, your racing monthly horoscope is:",
                        msg.author.id
                    ))
                    .embed(|embed| {
                        embed
                            .title(value)
                            .description(zodiac_contents)
                            .color(Color::DARK_BLUE)
                            .image(image_url)
                    })
            })
            .await?;
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
    msg.reply(ctx, "uwu").await?;

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let help_message = fs::read_to_string("content/help.txt")?;

    msg.channel_id
        .send_message(ctx, |help| {
            help.content(format!("<@{}>", msg.author.id))
                .embed(|embed| {
                    embed
                        .title("Help")
                        .description(help_message)
                        .color(Color::LIGHT_GREY)
                        .image("https://i.imgur.com/j0ROjd9.png")
                })
        })
        .await?;
    Ok(())
}

#[command]
async fn deleteme(ctx: &Context, msg: &Message) -> CommandResult {
    // Pulling in psql client
    let read = ctx.data.read().await;
    let client = read
        .get::<ZodiacClient>()
        .expect("PSQL client error")
        .clone();
    let user_id = msg.author.id.as_u64().to_string(); // User_ID to be used as the key

    let _statement = client
        .execute("DELETE FROM user_sign WHERE user_id = $1", &[&user_id])
        .await
        .expect("broken insert");
    let reply = format!("<@{}> You have been deleted!", msg.author.id);
    msg.channel_id.say(ctx, reply).await?;

    Ok(())
}
