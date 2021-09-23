use super::*;

#[command]
#[description = "Repeats what it's given, but without mentioning anything"]
pub async fn echo(ctx: &client::Context, msg: &Message, args: Args) -> CommandResult {
    // Repeat what the user said but without any mentions
    let content_safe_options = ContentSafeOptions::default();
    let clean_content = content_safe(ctx, &args.rest(), &content_safe_options).await;
    msg.reply(&ctx, clean_content).await?;
    Ok(())
}
