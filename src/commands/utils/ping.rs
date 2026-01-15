use serenity::all::{ChannelType, CommandInteraction, CommandOptionType, Context, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::builder::CreateCommand;

pub const COMMAND_NAME: &str = "utils ping";

pub fn register() -> CreateCommand {
    CreateCommand::new("utils")
        .description("Utils commands")
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand, "ping", "Ping pong!")
            .channel_types(vec![ChannelType::Text]))
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    let msg = CreateInteractionResponseMessage::new()
        .content("Pong!")
        .ephemeral(true);
    let msg = CreateInteractionResponse::Message(msg);
    interaction.create_response(&ctx.http, msg).await.expect("failed to send message");
}