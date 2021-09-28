use super::*;

#[command]
#[only_in(guilds)]
pub async fn faq(ctx: &client::Context, msg: &Message, args: Args) -> CommandResult {
    let db = ctx.get_db().await;
    let guild_id = msg.guild_id.context("couldn't get guild ID")?;

    match args.remains() {
        Some(title) => {
            let tag = db
                .get_faq_tag(guild_id, title)
                .await?
                .user_error(format!("Could not find \"{}\" in FAQ tags.", title).as_str())?;

            msg.reply_embed(ctx, |e| {
                e.title(tag.title);
                e.description(tag.content);
            })
            .await?;
        }
        None => {
            let faq_titles = db.get_all_faq_titles(guild_id).await?;
            if faq_titles.is_empty() {
                msg.reply_error(ctx, "This server does not have any defined FAQ tags.")
                    .await?;
                return Ok(());
            }

            msg.reply_embed(ctx, |e| {
                e.title("List of FAQ tags");
                e.description(faq_titles.join(", "));
            })
            .await?;
        }
    }

    Ok(())
}
