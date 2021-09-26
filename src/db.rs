use std::{env, sync::Arc};

use anyhow::Result;
use serenity::prelude::TypeMapKey;
use sqlx::SqlitePool;

pub struct Db {
    pub pool: SqlitePool,
}

impl TypeMapKey for Db {
    type Value = Arc<Db>;
}

impl Db {
    pub async fn new() -> Result<Self> {
        let pool = SqlitePool::connect(
            env::var("VBOT_DATABASE_URL")
                .expect("Please set VBOT_DATABASE_URL to point to an SQLite database file")
                .as_str(),
        )
        .await?;

        let created_db = Self { pool };
        created_db.migrate().await?;

        Ok(created_db)
    }

    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!()
            .run(&self.pool)
            .await
            .map_err(|err| err.into())
    }
}
