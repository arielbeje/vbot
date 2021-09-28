use super::*;

use serenity::framework::standard::CommandOptions;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserErr {
    #[error("Usage: {0}")]
    InvalidUsage(&'static str),

    #[error("{0}")]
    Other(String),
}

impl UserErr {
    pub fn invalid_usage(command_options: &CommandOptions) -> Self {
        Self::InvalidUsage(
            command_options
                .usage
                .unwrap_or("This isn't how you use this command"),
        )
    }

    pub fn other(s: &str) -> Self {
        Self::Other(s.to_string())
    }
}

pub trait OptionExt<T> {
    fn invalid_usage(self, command_options: &CommandOptions) -> Result<T, UserErr>;
    fn user_error(self, s: &str) -> Result<T, UserErr>;
}

impl<T> OptionExt<T> for Option<T> {
    fn invalid_usage(self, command_options: &CommandOptions) -> Result<T, UserErr> {
        self.ok_or_else(|| UserErr::InvalidUsage(command_options.usage.unwrap_or("")))
    }

    fn user_error(self, s: &str) -> Result<T, UserErr> {
        self.ok_or_else(|| UserErr::Other(s.to_string()))
    }
}

impl<T, E: Into<anyhow::Error>> OptionExt<T> for Result<T, E> {
    fn invalid_usage(self, command_options: &CommandOptions) -> Result<T, UserErr> {
        self.map_err(|_| UserErr::InvalidUsage(command_options.usage.unwrap_or("")))
    }

    fn user_error(self, s: &str) -> Result<T, UserErr> {
        self.map_err(|_| UserErr::Other(s.to_string()))
    }
}
