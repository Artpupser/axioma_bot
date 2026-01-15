#![allow(clippy::all, clippy::pedantic, clippy::manual_midpoint, clippy::allow_attributes)]
mod commands;
use std::env;
use serenity:: {
    async_trait, Client,
    client:: EventHandler,
    all::{GuildId, Interaction, Context, GatewayIntents, Ready }
};
struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("[{}] {} is connected!", &data_about_bot.user.id, &data_about_bot.user.name);
        let guild_id = env::var("BOT_GUID_ID").unwrap().parse::<u64>().expect("Failed to parse BOT_GUID_ID");
        let guild_id = GuildId::new(guild_id);
        let commands = guild_id
            .set_commands(&ctx.http, commands::CommandManager::all_commands())
            .await.unwrap();
        println!("Registered commands: {}", commands
            .into_iter()
            .map(|c| c.name)
            .collect::<Vec<String>>()
            .join(", "))
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            let mut cmd_full_name = String::new();
            cmd_full_name.push_str(&cmd.data.name);
            cmd_full_name.push(' ');
            cmd_full_name.push_str(&cmd.data.options.first().expect("Failed found sub command").name);
            commands::CommandManager::run_by_name(&cmd_full_name, &ctx, &cmd).await;
        }
    }
}
#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let shards_count = env::var("BOT_SHARDS_COUNT")
        .expect("BOT_SHARDS_COUNT is not set")
        .parse::<u32>().expect("Failed to parse BOT_SHARDS_COUNT");
    let token = env::var("BOT_TOKEN").expect("BOT_TOKEN is not set");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    println!("Axioma bot starting...");
    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");
    if let Err(err) = client.start_shards(shards_count).await {
        println!("Client error: {:?}", err);
    }
}