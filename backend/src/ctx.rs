use anyhow::Context;

use crate::db;

#[derive(Clone)]
pub struct Env {
    pub database_url: String,
}

impl Env {
    pub fn init() -> Env {
        let database_url = std::env::var("DATABASE_URL")
            .context("DATABASE_URL environment variable is not set")
            .unwrap();

        Env { database_url }
    }
}

#[derive(Clone)]
pub struct Ctx {
    pub env: Env,
    pub db: db::Db,
}

impl Ctx {
    pub async fn init() -> Ctx {
        let env = Env::init();
        let db = db::Db::connect(&env.database_url).await.unwrap();
        
        Self { env, db }
    }
}
