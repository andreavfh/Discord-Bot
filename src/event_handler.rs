use serenity::all::*;
use async_trait::async_trait;
use crate::command::all_slash_commands;

/// Trait for creating modular event handlers.
///
/// Implement this trait to hook into bot events like `on_message` and `on_ready`.
/// Default implementations do nothing, so you only need to override the methods you need.
#[async_trait]
pub trait BotEventHandler: Sync + Send {
    /// Called when a message is received.
    async fn on_message(&self, _ctx: &Context, _msg: &Message) {}

    /// Called when the bot is ready.
    async fn on_ready(&self, _ctx: &Context, _ready: &Ready) {}
}

/// Trait for types that have a static instance used for event registration.
///
/// Implement this for your struct and define a `const INSTANCE` to register it via the macro.
pub trait HasInstance {
    /// The static instance of your event handler.
    const INSTANCE: Self;
}

/// Macro to register a bot event handler using `inventory`.
///
/// Use this macro at the end of your module to register your event handler automatically:
///
/// ```
/// register_bot_event_handler!(MyEventHandler);
/// ```
#[macro_export]
macro_rules! register_bot_event_handler {
    ($handler:ty) => {
        inventory::submit! {
            &<$handler as $crate::event_handler::HasInstance>::INSTANCE
                as &'static (dyn $crate::event_handler::BotEventHandler + Sync + Send)
        }
    };
}

/// Collect all registered bot event handlers.
///
/// This is used internally by the main event dispatcher to call all handlers.
inventory::collect!(&'static (dyn BotEventHandler + Sync + Send));

/// Returns all collected event handlers.
pub fn all_event_handlers() -> Vec<&'static (dyn BotEventHandler + Sync + Send)> {
    let mut handlers = Vec::new();
    for handler in inventory::iter::<&'static (dyn BotEventHandler + Sync + Send)>() {
        handlers.push(*handler);
    }
    handlers
}

/// The main event handler for Serenity.
///
/// This handler delegates events to all registered `BotEventHandler` implementations.
/// You should pass an instance of `MainEventHandler` to Serenity's `ClientBuilder`.
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
