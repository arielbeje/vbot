use std::sync::Arc;

use serenity::{
    async_trait, builder::CreateEmbed, client::Context, model::channel::Message, utils::Colour, Result,
};

use crate::db::Db;

#[async_trait]
pub trait SerenityContextExt {
    async fn get_db(&self) -> Arc<Db>;
}

#[async_trait]
impl SerenityContextExt for Context {
    async fn get_db(&self) -> Arc<Db> {
        self.data.read().await.get::<Db>().unwrap().clone()
    }
}

#[async_trait]
pub trait MessageExt {
    async fn reply_embed<F>(&self, ctx: &Context, builder: F) -> Result<Message>
    where
        F: FnOnce(&mut CreateEmbed) + Send + Sync;

    async fn reply_error(&self, ctx: &Context, error: &str) -> Result<Message>;
}

#[async_trait]
impl MessageExt for Message {
    async fn reply_embed<F>(&self, ctx: &Context, builder: F) -> Result<Message>
    where
        F: FnOnce(&mut CreateEmbed) + Send + Sync,
    {
        let mut embed = CreateEmbed::default();
        builder(&mut embed);

        self.channel_id
            .send_message(&ctx, move |m| {
                m.allowed_mentions(|f| f.replied_user(false));
                m.reference_message(self);
                m.set_embed(embed)
            })
            .await
    }

    async fn reply_error(&self, ctx: &Context, error: &str) -> Result<Message> {
        self.reply_embed(ctx, |e| {
            e.title("Error");
            e.description(error);
            e.color(Colour::RED);
        })
        .await
    }
}
