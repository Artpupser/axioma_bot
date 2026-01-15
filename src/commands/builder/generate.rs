use serenity::all::{ChannelType, CommandInteraction, CommandOptionType, Context, CreateCommandOption};
use serenity::builder::CreateCommand;

pub const COMMAND_NAME : &str = "builder generate";

pub fn register() -> CreateCommand {
    CreateCommand::new("builder")
        .description("Builder server commands")
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand, "generate", "set file configuration, for fast generate server"
        ).add_sub_option(CreateCommandOption::new(
            CommandOptionType::String, "config_str", "configuration for generate server"
        ).required(true).max_length(5012).min_length(64)).channel_types(vec![ChannelType::Text]))
}

pub async fn run(_ctx: &Context, _interaction: &CommandInteraction) {
    let _config_str = _interaction.data.options.iter().find(|option| option.name == "config_str");

}