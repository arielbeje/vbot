use std::{env, sync::Arc};

use serenity::{
    async_trait,
    client::{self, bridge::gateway::GatewayIntents},
    framework::{
        standard::{macros::hook, CommandResult},
        StandardFramework,
    },
    http::Http,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use tracing::{error, info};

pub mod commands;
use commands::*;
pub mod db;
use db::Db;
pub mod extensions;
use extensions::*;

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

    let db = Db::new().await.expect("Failed to initialize database");

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
        .group(&GENERAL_GROUP)
        .group(&FAQ_GROUP)
        .after(after);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .intents(GatewayIntents::all())
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Db>(Arc::new(db));
    };

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

#[hook]
async fn after(ctx: &client::Context, msg: &Message, _command_name: &str, result: CommandResult) {
    if let Err(err) = result {
        if let Some(err) = err.downcast_ref::<UserErr>() {
            let _ = msg.reply_error(ctx, format!("{}", err).as_str()).await;
        }
    }
}
