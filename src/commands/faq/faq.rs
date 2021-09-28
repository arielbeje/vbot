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

#[command("new")]
#[aliases("add")]
#[only_in(guilds)]
#[usage("faq new <title> <conent>")]
pub async fn faq_new(ctx: &client::Context, msg: &Message, mut args: Args) -> CommandResult {
    let db = ctx.get_db().await;
    let guild_id = msg.guild_id.context("couldn't get guild ID")?;

    let title = args
        .single_quoted::<String>()
        .invalid_usage(&FAQ_NEW_COMMAND_OPTIONS)?;
    let content = args.remains().invalid_usage(&FAQ_NEW_COMMAND_OPTIONS)?;

    db.add_faq_tag(guild_id, title.as_str(), content).await?;

    msg.reply_embed(ctx, |e| {
        e.title(format!("Successfully added \"{}\" to database", title));
        e.description(content);
    })
    .await?;

    Ok(())
}
