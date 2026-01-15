use serenity::all::{CommandInteraction, Context, CreateCommand};

pub mod utils;
pub mod builder;

pub struct CommandManager;
impl CommandManager {
    pub fn all_commands() -> Vec<CreateCommand> {
        vec![
            utils::ping::register(),
            builder::generate::register(),
        ]
    }

    pub async fn run_by_name(name: &str, ctx: &Context, interaction: &CommandInteraction) {
        println!("Running command '{}'", name);
        match name {
            utils::ping::COMMAND_NAME => utils::ping::run(ctx, interaction).await,
            builder::generate::COMMAND_NAME => builder::generate::run(ctx, interaction).await,
            _ => (),
        }
    }
}