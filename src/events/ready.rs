use serenity::all::*;
use async_trait::async_trait;
use crate::command::register_global_slash_commands;
use crate::event_handler::{BotEventHandler, HasInstance};
use crate::register_bot_event_handler;

pub struct SlashReadyEvent;

impl HasInstance for SlashReadyEvent {
    const INSTANCE: Self = SlashReadyEvent;
}

#[async_trait]
impl BotEventHandler for SlashReadyEvent {
    async fn on_ready(&self, ctx: &Context, ready: &Ready) {
        println!("Bot ready as {}", ready.user.name);

        if let Err(err) = register_global_slash_commands(ctx).await {
            eprintln!("Error registering slash commands: {err:?}");
        } else {
            println!("Slash commands registered successfully.");
        }
    }
}

register_bot_event_handler!(SlashReadyEvent);