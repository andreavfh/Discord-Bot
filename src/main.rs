#[macro_use]
extern crate inventory;

mod command;
mod commands;
mod event_handler;
mod events;

use event_handler::MainEventHandler;
use serenity::all::*;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN env variable");

    let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(MainEventHandler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Error creating client {why:?}");
    }
}