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
    fn config(&self) -> &Config;
}

pub struct DotEnvConfigProvider(Config);

impl DotEnvConfigProvider {
    pub fn new() -> Self {
        use dotenv::dotenv;
        use std::env;
        dotenv().ok();
        let config = Config {
            home_uri: env::var("HOME_URI").expect("Missing config"),
            callback_url: env::var("CALLBACK_URL").expect("Missing config"),
            auth_uri: env::var("AUTH_URI").expect("Missing config"),
            base_url: env::var("BASE_URL").expect("Missing config"),
            discover_playlist: env::var("DISCOVER_PLAYLIST").expect("Missing config"),
            spotify_client_id: env::var("SPOTIFY_CLIENT_ID").expect("Missing config"),
            spotify_client_secret: env::var("SPOTIFY_CLIENT_SECRET").expect("Missing config"),
            redis_url: env::var("REDIS_URL").expect("Missing config"),
            redis_port: env::var("REDIS_PORT").expect("Missing config"),
            redis_db: env::var("REDIS_DB").expect("Missing config"),
            redis_password: env::var("REDIS_PASSWORD").expect("Missing config"),
        };

        DotEnvConfigProvider(config)
    }
}

impl ConfigProvider for DotEnvConfigProvider {
    fn config(&self) -> &Config {
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
    pub fn new(args: Vec<String>, argv: HashMap<String, Vec<String>>) -> Self {
        let db_name = args.get(1).expect("Missing config");
        let db_password = args.get(2).expect("Missing config");
        let home_uri = argv.get("home_uri").expect("Missing config").to_vec();
        let client_id = argv.get("client_id").expect("Missing config").to_vec();
        let config = Config {
            home_uri: db_name.to_string(),
            callback_url: db_password.to_string(),
            auth_uri: home_uri.first().expect("Missing config").to_string(),
            base_url: client_id.first().expect("Missing config").to_string(),
            ..Default::default()
        };
        CmdConfigProvider(config)
    }
}

impl ConfigProvider for CmdConfigProvider {
    fn config(&self) -> &Config {
        &self.0
    }
}

impl Default for CmdConfigProvider {
    fn default() -> Self {
        Self::new(Vec::new(), HashMap::new())
    }
}

pub struct EnvVarProvider(Config);

impl EnvVarProvider {
    pub fn new(_args: HashMap<String, String>) -> Self {
        let config = Config {
            ..Default::default()
        };
        EnvVarProvider(config)
    }
}

impl ConfigProvider for EnvVarProvider {
    fn config(&self) -> &Config {
        &self.0
    }
}

impl Default for EnvVarProvider {
    fn default() -> Self {
        Self::new(HashMap::new())
    }
}
