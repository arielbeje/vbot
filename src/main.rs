use std::env;

use serenity::{
    async_trait, client::bridge::gateway::GatewayIntents, framework::StandardFramework, http::Http,
    model::gateway::Ready, prelude::*,
};
use tracing::{error, info};

pub mod commands;
use commands::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} connected to Discord!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = env::var("VBOT_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);
    let app_info = http
        .get_current_application_info()
        .await
        .expect("Couldn't access application info");

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(app_info.id))
                .prefix(",")
        })
        .help(&help::VBOT_HELP)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .intents(GatewayIntents::all())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
