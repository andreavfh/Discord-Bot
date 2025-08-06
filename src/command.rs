use serenity::all::*;
use async_trait::async_trait;

/// A trait that defines a global slash command for a Discord bot using Serenity.
///
/// Each struct implementing this trait can be dynamically registered and executed.
/// Useful in modular bot architectures.
///
/// Use the `register_slash_command!` macro to automatically register the command
/// via the inventory system.
#[async_trait]
pub trait SlashCommand: Sync + Send {
    /// The name of the slash command (e.g. `"ping"`).
    ///
    /// This will be the trigger used by users, such as `/ping`.
    fn name(&self) -> &'static str;

    /// A short description of what the command does.
    ///
    /// This is shown in the Discord client when browsing commands.
    fn description(&self) -> &'static str;

    /// (Optional) Returns the list of command options (parameters) used by this command.
    ///
    /// Override this if your command uses options like strings, integers, booleans, etc.
    /// These will be included in the `register()` method automatically.
    ///
    /// Default is an empty list (no options).
    fn options(&self) -> Vec<CreateCommandOption> {
        vec![]
    }

    /// Defines how this command should be registered on Discord.
    ///
    /// This uses `name()`, `description()`, and `options()` by default.
    /// You can override this if you need advanced customization.
    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description(self.description())
            .set_options(self.options())
    }

    /// The logic to be executed when this command is invoked.
    ///
    /// # Arguments
    /// * `ctx` - The bot context provided by Serenity.
    /// * `interaction` - The interaction object representing the command usage.
    async fn run(&self, ctx: &Context, interaction: &CommandInteraction);
}

/// A helper trait to provide a static reference to an instance of the command.
pub trait HasInstance {
    const INSTANCE: Self;
}

/// Macro to register a struct that implements `SlashCommand` and `HasInstance`.
///
/// Usage:
/// ```
/// register_slash_command!(MyCommandStruct);
/// ```
#[macro_export]
macro_rules! register_slash_command {
    ($command:ty) => {
        inventory::submit! {
            &< $command as $crate::command::HasInstance >::INSTANCE
                as &'static (dyn $crate::command::SlashCommand + Sync + Send)
        }
    };
}

// Collect all registered slash commands from inventory
inventory::collect!(&'static (dyn SlashCommand + Sync + Send));

/// Returns a list of all slash commands registered in the inventory.
pub fn all_slash_commands() -> Vec<&'static (dyn SlashCommand + Sync + Send)> {
    inventory::iter::<&'static (dyn SlashCommand + Sync + Send)>
        .into_iter()
        .copied()
        .collect()
}

/// Registers all collected slash commands globally with Discord.
///
/// This will call `register()` on each command, which now includes name, description, and options.
pub async fn register_global_slash_commands(ctx: &Context) -> Result<(), serenity::Error> {
    let commands: Vec<CreateCommand> = all_slash_commands()
        .iter()
        .map(|cmd| cmd.register())
        .collect();

    Command::set_global_commands(&ctx.http, commands).await?;
    Ok(())
}
