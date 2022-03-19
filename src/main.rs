use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const HELP_MSG: &str = "Hello I am help";
const HELP_CMD: &str = "!help";

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

    let mut client = Client::builder(&token)
        .event_handler(MessageHandler)
        .await
        .expect("Error creating client");

    if let Err(error) = client.start().await {
        println!("Client error {:?}", error)
    }
}
