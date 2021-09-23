use super::*;

use std::collections::HashSet;

use serenity::framework::standard::{help_commands, macros::help, CommandGroup, HelpOptions};

#[help]
#[lacking_permissions = "Hide"]
#[max_levenshtein_distance(3)]
pub async fn vbot_help(
    ctx: &client::Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}
