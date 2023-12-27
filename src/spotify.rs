use std::collections::HashMap;
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

struct Playlists {
    playlists: <Vec<Playlist> as IntoIterator>::IntoIter,
    client: reqwest::blocking::Client,
    limit: u32,
    offset: u32,
    next: String,
    previous: String,
    total: u32
}

impl Playlists {
    fn new() -> Result<Self> {
        Ok(Playlists{
            playlists: vec![].into_iter(),
            client: reqwest::blocking::Client::new(),
            limit: 50,
            offset: 0,
            next: "".to_owned(),
            previous: "".to_owned(),
            total: 0
        })
    }
    fn try_next(&mut self) -> Result<Option<Playlist>> {

    }
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

    async fn get<T>(&self, mut uri: String, query_params: HashMap<&str, i32>) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let mut ssl_builder = SslConnector::builder(SslMethod::tls()).unwrap();
        ssl_builder.set_verify(SslVerifyMode::NONE);
        let client = awc::Client::builder()
            .connector(Connector::new().openssl(ssl_builder.build()))
            .finish();
        let mut params = "?".to_owned();
        for (key, value) in query_params {
            params.push_str(&format!("{key}={value}&"));
        }
        params.pop(); // Remove last & or ?
        uri.push_str(&params);
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
    pub async fn discovery_weekly_playlist(&self, config: &Config) -> Result<Playlist> {
        let query_params = HashMap::from([
            ("limit", 50)
        ]);
        let base_url = &config.base_url;
        let uri = base_url.to_owned() + "/playlists/" + &config.discover_playlist;
        self.get::<Playlist>(uri, query_params).await
    }
    pub async fn get_or_create_archive_playlist(&self, config: &Config) -> Result<Playlist> {
        let base_url = &config.base_url;
        let query_params = HashMap::from([
            ("limit", 50)
        ]);
        let uri = base_url.to_owned() + "/me/playlists";
        self.get::<Playlist>(uri, query_params).await
    }
}
