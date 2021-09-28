use anyhow::{Context, Result};
use serenity::{
    client,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{
        id::{GuildId, UserId},
        prelude::*,
    },
    utils::{content_safe, ContentSafeOptions},
};

use crate::{db::Db, extensions::*};

pub mod errors;
pub use errors::*;

pub mod echo;
use echo::*;
pub mod faq;
use faq::*;
pub mod help;

#[group]
#[prefix("faq")]
#[commands(faq_new)]
#[default_command(faq)]
struct Faq;

#[group]
#[commands(echo)]
struct General;
