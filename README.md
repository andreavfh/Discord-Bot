# 🪴 Slash Commands

This project uses a custom `SlashCommand` trait to easily define, register, and run slash commands in a modular way, powered by:

- [`serenity`](https://docs.rs/serenity) 0.12
- `#[async_trait]`
- [`inventory`](https://docs.rs/inventory) for auto-registration

No need to manually collect or register commands — just implement the trait, use a macro, and you're done!

---

---

## 📦 Example: `/sum` Command 

This example defines a slash command called `/sum` that adds two numbers and replies with the result.

###  Code

```rust
use serenity::all::*;
use async_trait::async_trait;
use crate::command::{SlashCommand, HasInstance};
use crate::register_slash_command;

pub struct SumCommand;

impl HasInstance for SumCommand {
    const INSTANCE: Self = SumCommand;
}

#[async_trait]
impl SlashCommand for SumCommand {
    fn name(&self) -> &'static str { "sum" }

    fn description(&self) -> &'static str { "Adds two numbers" }

    fn options(&self) -> Vec<CreateCommandOption> {
        vec![
            CreateCommandOption::new(CommandOptionType::Integer, "a", "First number").required(true),
            CreateCommandOption::new(CommandOptionType::Integer, "b", "Second number").required(true),
        ]
    }

    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) {
        let a = interaction.data.options.get(0).and_then(|o| o.value.as_ref())?.as_i64().unwrap_or(0);
        let b = interaction.data.options.get(1).and_then(|o| o.value.as_ref())?.as_i64().unwrap_or(0);
        let _ = interaction.create_response(ctx, CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(format!("Result: {}", a + b))
        )).await;
    }
}

register_slash_command!(SumCommand);
```

---

## 📁 Folder Structure Suggestion

```
src/
├── command/
│   ├── mod.rs             # Your trait, inventory setup, macro, and register logic
│   ├── commands/
│   │   ├── sum.rs         # Your individual command implementations
│   │   └── ...
```

---

## 🧠 How It Works

1. Each command implements the `SlashCommand` trait.
2. You define the command's name, description, options, and logic.
3. The `HasInstance` trait is used to provide a static instance of the command.
4. Use `register_slash_command!(YourCommand)` to submit it into the inventory system.
5. When the bot starts, it can automatically register all commands using a single function.

---

## ✅ Final Result

- Command: `/sum`
- Input: `/sum a:5 b:7`
- Bot response: `Result: 12`

---

# 🧠 Custom Event Handlers

This project includes a flexible and modular system to handle Discord events using Serenity. Events like `on_message`, `on_ready`, or `interaction_create` are processed through a trait-based system that supports **auto-registration** via [`inventory`](https://docs.rs/inventory).

---

## ✨ Features

- ✅ Automatically register event listeners using a macro
- ✅ Modular design: implement only the events you need
- ✅ Shared trait (`BotEventHandler`) for all event types
- ✅ Easily add new types of events in the future

---

## 🔌 How It Works

### 1. `BotEventHandler` Trait

This is the core trait. Any struct implementing this trait can hook into one or more Serenity events:

```rust
#[async_trait]
pub trait BotEventHandler: Sync + Send {
    async fn on_message(&self, _ctx: &Context, _msg: &Message) {}
    async fn on_ready(&self, _ctx: &Context, _ready: &Ready) {}
}
```

- All methods have default implementations, so you can implement only what you need.
- More methods (like `on_guild_create`, `on_reaction_add`, etc.) can be added as needed.

---

### 2. `HasInstance` Trait

Each handler must implement this to provide a static instance of itself:

```rust
pub trait HasInstance {
    const INSTANCE: Self;
}
```

---

### 3. Registering an Event Handler

Use the provided macro to register your handler via `inventory`:

```rust
#[macro_export]
macro_rules! register_bot_event_handler {
    ($handler:ty) => {
        inventory::submit! {
            &<$handler as $crate::event_handler::HasInstance>::INSTANCE
                as &'static (dyn $crate::event_handler::BotEventHandler + Sync + Send)
        }
    };
}
```

Example usage:

```rust
register_bot_event_handler!(MyMessageLogger);
```

---

### 4. Collecting All Handlers

Handlers are collected at runtime using the `inventory` crate:

```rust
pub fn all_event_handlers() -> Vec<&'static (dyn BotEventHandler + Sync + Send)> {
    inventory::iter::<&'static (dyn BotEventHandler + Sync + Send)>().copied().collect()
}
```

---

## 🧠 MainEventHandler: Delegating Events

This struct implements Serenity’s `EventHandler`, and forwards events to all registered `BotEventHandler`s:

```rust
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
```

This allows you to cleanly separate each event’s logic into its own file or module.

---

## ✅ Example: Logging Messages

```rust
use serenity::all::*;
use async_trait::async_trait;
use crate::event_handler::{BotEventHandler, HasInstance};
use crate::register_bot_event_handler;

pub struct MessageLogger;

impl HasInstance for MessageLogger {
    const INSTANCE: Self = MessageLogger;
}

#[async_trait]
impl BotEventHandler for MessageLogger {
    async fn on_message(&self, _ctx: &Context, msg: &Message) {
        if !msg.author.bot {
            println!("User {} said: {}", msg.author.name, msg.content);
        }
    }
}

register_bot_event_handler!(MessageLogger);
```

---

## 🧩 Adding New Events

You can extend the system easily by adding more methods to the `BotEventHandler` trait:

```rust
async fn on_guild_create(&self, _ctx: &Context, _guild: &Guild) {}
```

Then update `MainEventHandler` to call those methods.

---

## 📁 Folder Structure Example

```
src/
├── event_handler.rs
└── events/
    ├── mod.rs
    └── message_logger.rs
```
---

## 📚 Dependencies Used

- `serenity = "0.12"`
- `inventory = "0.3"`
- `async-trait = "0.1"`

---

## ❓ Need Help?

Feel free to open an issue or ask questions if you want to:

- Use guild-only commands
- Add optional arguments
- Support subcommands or groups
