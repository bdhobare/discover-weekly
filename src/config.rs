#[derive(Debug, Default)]
pub struct Config {
    pub redirect_uri: String,
    pub auth_uri: String,
    pub base_url: String,
    pub discover_playlist: String,
    pub spotify_client_id: String,
    pub spotify_client_secret: String,
    pub redis_url: String,
    pub redis_port: String,
    pub redis_db: String,
    pub redis_password: String,
}

pub trait ConfigProvider {
    fn get_config(&self) -> &Config;
}

pub struct DotEnvConfigProvider(Config);

impl DotEnvConfigProvider {
    pub fn new() -> Self {
        use dotenv::dotenv;
        use std::env;
        dotenv().ok();
        let config = Config {
            redirect_uri: env::var("REDIRECT_URI").expect("Missing redirect uri"),
            auth_uri: env::var("AUTH_URI").expect("Missing auth uri"),
            base_url: env::var("BASE_URL").expect("Missing base uri"),
            discover_playlist: env::var("DISCOVER_PLAYLIST").expect("Missing discover playlist"),
            spotify_client_id: env::var("SPOTIFY_CLIENT_ID").expect("Missing spotify client id"),
            spotify_client_secret: env::var("SPOTIFY_CLIENT_SECRET")
                .expect("Missing spotify client secret"),
            redis_url: env::var("REDIS_URL").expect("Missing redis uri"),
            redis_port: env::var("REDIS_PORT").expect("Missing redis port"),
            redis_db: env::var("REDIS_DB").expect("Missing redis db"),
            redis_password: env::var("REDIS_PASSWORD").expect("Missing redis password"),
        };

        DotEnvConfigProvider(config)
    }
}

impl ConfigProvider for DotEnvConfigProvider {
    fn get_config(&self) -> &Config {
        &self.0
    }
}

impl Default for DotEnvConfigProvider {
    fn default() -> Self {
        Self::new()
    }
}
