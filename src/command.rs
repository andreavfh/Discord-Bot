use serenity::all::*;
use async_trait::async_trait;

#[async_trait]
pub trait SlashCommand: Sync + Send {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn register(&self) -> CreateCommand;
    async fn run(&self, ctx: &Context, interaction: &CommandInteraction);
}

pub trait HasInstance {
    const INSTANCE: Self;
}

#[macro_export]
macro_rules! register_slash_command {
    ($command:ty) => {
        inventory::submit! {
            &< $command as $crate::command::HasInstance >::INSTANCE
                as &'static (dyn $crate::command::SlashCommand + Sync + Send)
        }
    };
}

inventory::collect!(&'static (dyn SlashCommand + Sync + Send));

pub fn all_slash_commands() -> Vec<&'static (dyn SlashCommand + Sync + Send)> {
    inventory::iter::<&'static (dyn SlashCommand + Sync + Send)>
        .into_iter().copied()
        .collect()
}

pub async fn register_global_slash_commands(ctx: &Context) -> Result<(), serenity::Error> {
    let commands: Vec<CreateCommand> = all_slash_commands()
        .iter()
        .map(|cmd| cmd.register())
        .collect();

    Command::set_global_commands(&ctx.http, commands).await?;
    Ok(())
}


