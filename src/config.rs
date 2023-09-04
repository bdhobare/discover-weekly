use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Config {
    pub home_uri: String,
    pub callback_url: String,
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
            home_uri: env::var("HOME_URI").expect("Missing redirect uri"),
            callback_url: env::var("CALLBACK_URL").expect("Missing callback url"),
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

pub struct CmdConfigProvider(Config);

impl CmdConfigProvider {
    pub fn new(args: HashMap<String, Vec<String>>) -> Self {
        let home_uri: Vec<String> = args.get("home_uri").unwrap().to_vec();
        let config = Config {
            home_uri: home_uri.first().unwrap().to_string(),
            ..Default::default()
        };
        CmdConfigProvider(config)
    }
}

impl ConfigProvider for CmdConfigProvider {
    fn get_config(&self) -> &Config {
        &self.0
    }
}

impl Default for CmdConfigProvider {
    fn default() -> Self {
        Self::new(HashMap::new())
    }
}

pub struct EnvVarProvider(Config);

impl EnvVarProvider {
    pub fn new(args: HashMap<String, String>) -> Self {
        let config = Config {
            home_uri: args.get("HOME_URI").unwrap().to_string(),
            ..Default::default()
        };
        EnvVarProvider(config)
    }
}

impl ConfigProvider for EnvVarProvider {
    fn get_config(&self) -> &Config {
        &self.0
    }
}

impl Default for EnvVarProvider {
    fn default() -> Self {
        Self::new(HashMap::new())
    }
}
