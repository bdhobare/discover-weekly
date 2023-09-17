use crate::auth::{AuthResponse, FetchToken, TokenResponse};
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

#[derive(Serialize, Deserialize, Default)]
pub struct Track {
    id: String,
    name: String,
    uri: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TrackItem {
    added_at: String,
    track: Track,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PlaylistTrackItem {
    href: String,
    items: Vec<TrackItem>,
    total: u32,
}
#[derive(Serialize, Deserialize, Default)]
pub struct Playlist {
    id: String,
    uri: String,
    name: String,
    description: Option<String>,
    tracks: PlaylistTrackItem,
}

#[derive(Clone)]
pub struct SpotifyClient;

#[async_trait(?Send)]
impl FetchToken for SpotifyClient {
    async fn fetch_token(&self, config: &Config, code: &AuthResponse) -> Result<TokenResponse> {
        let mut res = Self::get_token(code, config).await?;
        let json = res.json::<serde_json::Value>().await?;
        if let Ok(auth_response) = serde_json::from_value::<TokenResponse>(json.clone()) {
            return Ok(auth_response);
        }
        Err(SpotifyError::Unknown {
            msg: json.to_string().into(),
        })
    }
}

impl Default for SpotifyClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SpotifyClient {
    pub fn new() -> SpotifyClient {
        SpotifyClient {}
    }

    async fn get_token(
        code: &AuthResponse,
        config: &Config,
    ) -> Result<ClientResponse<Decoder<Payload>>, SendRequestError> {
        let params = [
            ("grant_type", "authorization_code"),
            ("code", &code.code.clone().unwrap_or_default()),
            ("redirect_uri", &config.callback_url),
        ];
        let client_id = &config.spotify_client_id.to_string();
        let client_secret = &config.spotify_client_secret;
        let encoded =
            general_purpose::STANDARD_NO_PAD.encode(client_id.to_string() + ":" + client_secret);
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
    pub async fn get_discovery_weekly_playlist(
        self,
        config: &Config,
        bearer: &str,
    ) -> Result<Playlist> {
        let base_url = &config.base_url;
        let uri = base_url.to_owned() + "/playlists/" + &config.discover_playlist;
        let mut ssl_builder = SslConnector::builder(SslMethod::tls()).unwrap();
        ssl_builder.set_verify(SslVerifyMode::NONE);
        let client = awc::Client::builder()
            .connector(Connector::new().openssl(ssl_builder.build()))
            .finish();
        let mut response = client
            .get(uri)
            .insert_header(("Authorization", "Bearer ".to_owned() + bearer))
            .send()
            .await?;
        let json = response.json::<serde_json::Value>().await?;
        if let Ok(playlist) = serde_json::from_value::<Playlist>(json) {
            return Ok(playlist);
        }
        Err(SpotifyError::CantFetchPlaylist(
            "Error fetching playlist".into(),
        ))
    }
    pub async fn get_or_create_archive_playlist(self, config: &Config, bearer: &str) -> Result<Playlist> {
        let base_url = &config.base_url;
        let uri = base_url.to_owned() + "/me/playlists?limit=50";
        let mut ssl_builder = SslConnector::builder(SslMethod::tls()).unwrap();
        ssl_builder.set_verify(SslVerifyMode::NONE);
        let client = awc::Client::builder()
            .connector(Connector::new().openssl(ssl_builder.build()))
            .finish();
        let mut response = client
            .get(uri)
            .insert_header(("Authorization", "Bearer ".to_owned() + bearer))
            .send()
            .await?;
        let json = response.json::<serde_json::Value>().await?;
        if let Ok(playlist) = serde_json::from_value::<Playlist>(json) {
            return Ok(playlist);
        }
        Err(SpotifyError::CantFetchPlaylist(
            "Error fetching playlist".into(),
        ))
    }
}
