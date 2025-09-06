use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use anyhow::Context;
#[derive(Clone)]
pub struct Db {
    conn: Pool<Postgres>,
}

impl Db {
    pub async fn connect(database_url: &str) -> anyhow::Result<Self> {
        let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("Failed to connect to the database")?;

        Ok(Db { conn })
    }

    pub fn get_conn(&self) -> &Pool<Postgres> {
        &self.conn
    }

    #[allow(dead_code)]
    pub async fn migrate(&self) -> anyhow::Result<()> {
        sqlx::migrate!()
            .run(&self.conn)
            .await
            .context("Failed to run database migrations")?;
        Ok(())
    }
}