use serenity::{
    client,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::prelude::*,
    utils::{content_safe, ContentSafeOptions},
};

pub mod echo;
use echo::*;
pub mod help;

#[group]
#[commands(echo)]
struct General;
