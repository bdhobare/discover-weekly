use crate::auth::TokenResponse;
use crate::config::Config;
use crate::spotify_error::{Result, SpotifyError};
use chrono::prelude::*;
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Store {
    con: Arc<Mutex<redis::aio::Connection>>,
    redis_key: String,
    insert_time_key: String,
}

impl Store {
    pub async fn open(config: &Config) -> Result<Store> {
        let client = redis::Client::open(Store::get_connection_info(
            &config.redis_password,
            &config.redis_url,
            &config.redis_port,
            &config.redis_db,
        ))?;
        let con = client.get_async_connection().await?;
        Ok(Store {
            con: Arc::new(Mutex::new(con)),
            redis_key: "tokens".to_string(),
            insert_time_key: "tokens:time".to_string(),
        })
    }
    pub async fn store_access_tokens(self, token_response: &TokenResponse) -> Result<()> {
        let tokens = serde_json::to_string(token_response)?;
        let mut guard = self.con.lock().await;
        guard.set(self.redis_key, tokens).await?;
        guard
            .set(self.insert_time_key, Utc::now().to_string())
            .await?;
        Ok(())
    }
    pub async fn get_token_response(self) -> Result<(TokenResponse, String)> {
        let mut guard = self.con.lock().await;
        let tokens: Option<String> = guard.get(self.redis_key).await?;
        let _insert_time: Option<String> = guard.get(self.insert_time_key).await?;
        if let Some(value) = tokens {
            let response: TokenResponse = serde_json::from_str(&value)?;
            return Ok((response, String::default()));
        }
        Err(SpotifyError::Unknown {
            msg: "Missing value".to_string().into(),
        })
    }
    fn get_connection_info(
        redis_password: &str,
        redis_host: &str,
        redis_port: &str,
        redis_db: &String,
    ) -> String {
        use urlencoding::encode;
        format!(
            "redis://:{}@{}:{}/{}",
            encode(redis_password),
            redis_host,
            redis_port,
            redis_db
        )
    }
}
