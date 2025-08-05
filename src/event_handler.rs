use serenity::all::*;
use async_trait::async_trait;
use crate::command::all_slash_commands;

#[async_trait]
pub trait BotEventHandler: Sync + Send {
    async fn on_message(&self, _ctx: &Context, _msg: &Message) {}
    async fn on_ready(&self, _ctx: &Context, _ready: &Ready) {}
}

pub trait HasInstance {
    const INSTANCE: Self;
}

#[macro_export]
macro_rules! register_bot_event_handler {
    ($handler:ty) => {
        inventory::submit! {
            &<$handler as $crate::event_handler::HasInstance>::INSTANCE
                as &'static (dyn $crate::event_handler::BotEventHandler + Sync + Send)
        }
    };
}

inventory::collect!(&'static (dyn BotEventHandler + Sync + Send));

pub fn all_event_handlers() -> Vec<&'static (dyn BotEventHandler + Sync + Send)> {
    let mut handlers = Vec::new();
    for handler in inventory::iter::<&'static (dyn BotEventHandler + Sync + Send)>() {
        handlers.push(*handler);
    }
    handlers
}

pub struct MainEventHandler;

#[async_trait]
impl EventHandler for MainEventHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        for handler in all_event_handlers() {
            handler.on_message(&ctx, &msg).await;
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        for handler in all_event_handlers() {
            handler.on_ready(&ctx, &ready).await;
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command_interaction) = interaction {
            for cmd in all_slash_commands() {
                if cmd.name() == command_interaction.data.name {
                    cmd.run(&ctx, &command_interaction).await;
                }
            }
        }
    }
}