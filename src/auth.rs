use crate::config::Config;
use crate::spotify_error::{Result, SpotifyError};
use actix_http::encoding::Decoder;
use actix_http::Payload;
use async_trait::async_trait;
use awc::error::SendRequestError;
use awc::{ClientResponse, Connector};
use base64::{engine::general_purpose, Engine as _};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct AuthClient;
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
    async fn fetch_token(
        &self,
        config: &Config,
        auth_response: &AuthResponse,
    ) -> Result<TokenResponse>;
}

#[async_trait(?Send)]
impl FetchToken for AuthClient {
    async fn fetch_token(
        &self,
        config: &Config,
        auth_response: &AuthResponse,
    ) -> Result<TokenResponse> {
        let mut res = Self::get_token(auth_response, config).await?;
        let json = res.json::<serde_json::Value>().await?;
        if let Ok(auth_response) = serde_json::from_value::<TokenResponse>(json.clone()) {
            return Ok(auth_response);
        }
        Err(SpotifyError::Unknown {
            msg: json.to_string().into(),
        })
    }
}

impl AuthClient {
    async fn get_token(
        auth_response: &AuthResponse,
        config: &Config,
    ) -> Result<ClientResponse<Decoder<Payload>>, SendRequestError> {
        let mut params = vec![
            ("grant_type", "authorization_code"),
            ("redirect_uri", &config.callback_url),
        ];
        if let Some(code) = &auth_response.code {
            params.push(("code", code.as_str()));
        }
        let client_id = &config.spotify_client_id;
        let client_secret = &config.spotify_client_secret;
        let encoded =
            general_purpose::STANDARD_NO_PAD.encode(client_id.to_owned() + ":" + client_secret);
        let mut ssl_builder = SslConnector::builder(SslMethod::tls()).unwrap();
        ssl_builder.set_verify(SslVerifyMode::NONE);
        let client = awc::Client::builder()
            .connector(Connector::new().openssl(ssl_builder.build()))
            .finish();
        client
            .post(&config.auth_uri)
            .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
            .insert_header(("Authorization", "Basic ".to_owned() + &encoded))
            .send_form(&params)
            .await
    }
}
