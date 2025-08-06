use crate::command::{SlashCommand, HasInstance};
use serenity::all::*;
use async_trait::async_trait;
use crate::register_slash_command;

pub struct PingCommand;

impl HasInstance for PingCommand {
    const INSTANCE: Self = PingCommand;
}

#[async_trait]
impl SlashCommand for PingCommand {
    fn name(&self) -> &'static str { "ping" }
    fn description(&self) -> &'static str { "Replies pong!" }
    fn register(&self) -> CreateCommand {
        CreateCommand::new(Self::name(self)).description(Self::description(self))
    }
    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) {
        let _ = interaction.create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("🏓 Pong!"),
            )
        ).await;
    }
}

register_slash_command!(PingCommand);