use super::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserErr {
    #[error("{0}")]
    Other(String),
}

impl UserErr {
    pub fn other(s: &str) -> Self {
        Self::Other(s.to_string())
    }
}

pub trait OptionExt<T> {
    fn user_error(self, s: &str) -> Result<T, UserErr>;
}

impl<T> OptionExt<T> for Option<T> {
    fn user_error(self, s: &str) -> Result<T, UserErr> {
        self.ok_or_else(|| UserErr::Other(s.to_string()))
    }
}

impl<T, E: Into<anyhow::Error>> OptionExt<T> for Result<T, E> {
    fn user_error(self, s: &str) -> Result<T, UserErr> {
        self.map_err(|_| UserErr::Other(s.to_string()))
    }
}
