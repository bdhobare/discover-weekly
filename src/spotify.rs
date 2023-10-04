use crate::auth::TokenResponse;
use crate::config::Config;
use crate::spotify_error::{Result, SpotifyError};
use awc::Connector;
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
pub struct SpotifyClient {
    auth_info: TokenResponse,
}

impl SpotifyClient {
    pub fn new(token_response: TokenResponse) -> SpotifyClient {
        SpotifyClient {
            auth_info: token_response,
        }
    }

    async fn get<T>(&self, uri: &str) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let mut ssl_builder = SslConnector::builder(SslMethod::tls()).unwrap();
        ssl_builder.set_verify(SslVerifyMode::NONE);
        let client = awc::Client::builder()
            .connector(Connector::new().openssl(ssl_builder.build()))
            .finish();
        let mut response = client
            .get(uri)
            .insert_header((
                "Authorization",
                "Bearer ".to_owned() + &self.auth_info.access_token,
            ))
            .send()
            .await?;
        let json = response.json::<serde_json::Value>().await?;
        if let Ok(result) = serde_json::from_value::<T>(json) {
            return Ok(result);
        }
        Err(SpotifyError::Unknown { msg: None })
    }
    pub async fn get_discovery_weekly_playlist(&self, config: &Config) -> Result<Playlist> {
        let base_url = &config.base_url;
        let uri = base_url.to_owned() + "/playlists/" + &config.discover_playlist;
        self.get::<Playlist>(&uri).await
    }
    pub async fn get_or_create_archive_playlist(&self, config: &Config) -> Result<Playlist> {
        let base_url = &config.base_url;
        let uri = base_url.to_owned() + "/me/playlists?limit=50";
        self.get::<Playlist>(&uri).await
    }
}
