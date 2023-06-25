use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::spotify_error::Result;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct AuthResponse {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub scope: String,
}

#[async_trait(?Send)]
pub trait FetchToken {
    async fn fetch_token(&self, config: &Config, code: &AuthResponse) -> Result<TokenResponse>;
}
