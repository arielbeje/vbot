use super::Db;
use super::*;

#[allow(clippy::module_inception)]
pub mod faq;
pub use faq::*;

#[derive(Debug)]
pub struct FaqTag {
    title: String,
    content: String,
}

#[derive(Debug)]
pub struct FaqTitle {
    id: i64,
    guild: GuildId,
    title: String,
    content_id: i64,
}

impl Db {
    pub async fn get_all_faq_titles(&self, guild_id: GuildId) -> Result<Vec<String>> {
        let mut conn = self.pool.acquire().await?;
        let guild = guild_id.0 as i64;

        Ok(sqlx::query_scalar!(
            "SELECT title FROM faq_titles WHERE guild=? ORDER BY title ASC",
            guild
        )
        .fetch_all(&mut conn)
        .await?)
    }

    pub async fn get_faq_tag(&self, guild_id: GuildId, tag: &str) -> Result<Option<FaqTag>> {
        let mut conn = self.pool.acquire().await?;
        let guild = guild_id.0 as i64;

        // Selecting the title to get its casing (since we're querying case-insensitively)
        Ok(sqlx::query!(
            "
SELECT faq_titles.title, faq_content.content
FROM faq_content
LEFT OUTER JOIN faq_titles
ON faq_titles.content_id=faq_content.id
WHERE faq_titles.guild=?
    AND faq_titles.title=?
            ",
            guild,
            tag
        )
        .fetch_optional(&mut conn)
        .await?
        .map(|rec| FaqTag {
            title: rec.title,
            content: rec.content,
        }))
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct FaqContent {
    id: i64,
    guild: id::GuildId,
    content: String,
}
