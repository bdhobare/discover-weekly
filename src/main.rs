use actix_web::web::Redirect;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use discover_weekly::auth::{AuthClient, AuthResponse, FetchToken};
use discover_weekly::config::{ConfigProvider, DotEnvConfigProvider};
use discover_weekly::spotify::SpotifyClient;
use discover_weekly::store::Store;
use log::info;
use std::env;

struct AppState {
    config_provider: Box<dyn ConfigProvider>,
}

#[get("/callback")]
async fn callback(data: web::Data<AppState>, response: web::Query<AuthResponse>) -> impl Responder {
    let auth_response = response.into_inner();
    let app_data = data.into_inner();
    let config_provider = &app_data.config_provider;
    let app_config = config_provider.get_config();
    if let Some(_code) = &auth_response.code {
        // User approved request,
        info!("Code successfully fetched");
        let auth_client = AuthClient;
        let auth_info = auth_client.fetch_token(app_config, &auth_response).await;
        match auth_info {
            Ok(token_response) => {
                let spotify_client = SpotifyClient::new(token_response.clone());
                let store = Store::open(app_config).await;
                let _ = store.unwrap().store_access_tokens(&token_response).await;
                let playlist = spotify_client
                    .get_discovery_weekly_playlist(app_config)
                    .await;
                return HttpResponse::Ok().body(serde_json::to_string(&playlist.unwrap()).unwrap());
            }
            Err(err) => return HttpResponse::Ok().body(err.to_string()),
        }
    }
    // Error occurred or user denied request
    let reply = serde_json::to_string(&auth_response).unwrap();
    info!("{}", &reply);
    HttpResponse::Ok().body(reply)
}
#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let callback_url = &data.config_provider.get_config().callback_url;
    let client_id = &data.config_provider.get_config().spotify_client_id;
    let redirect_url = "https://accounts.spotify.com/authorize?client_id=".to_owned()
        + client_id +"&response_type=code&redirect_uri=" + callback_url
        + "&scope=playlist-read-private playlist-modify-private user-read-recently-played&show_dialog=false";
    Redirect::to(redirect_url)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 2, "cargo run -- <HOST> <PORT>");
    let bind_address: &str = &args[1];
    let bind_port: u16 = args[2].parse().unwrap();
    info!("Open: {}:{}", bind_address, bind_port);
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                config_provider: Box::new(DotEnvConfigProvider::new()),
            }))
            .service(index)
            .service(callback)
    })
    .bind((bind_address, bind_port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use discover_weekly::spotify_error::Result;
    use discover_weekly::{auth::TokenResponse, config::Config};
    const TEST_RESPONSE: &str = r#"
        {
            "access_token": "BQDmaWDt6ToQqjXdKS9yx7zm9qk9r5Mb0",
            "token_type": "Bearer",
            "expires_in": 3600,
            "refresh_token": "AQCvsYjo2W2gudqNq04ozsXuz4fS97BLnHaS",
            "scope": "playlist-read-private playlist-modify-private user-read-recently-played"
        }
        "#;
    struct MockClient;
    #[async_trait(?Send)]
    impl FetchToken for MockClient {
        async fn fetch_token(
            &self,
            _config: &Config,
            _auth_response: &AuthResponse,
        ) -> Result<TokenResponse> {
            let response: TokenResponse = serde_json::from_str(&TEST_RESPONSE).unwrap();
            Ok(response)
        }
    }

    struct MockConfigProvider(Config);
    impl ConfigProvider for MockConfigProvider {
        fn get_config(&self) -> &Config {
            &self.0
        }
    }

    impl Default for MockConfigProvider {
        fn default() -> Self {
            MockConfigProvider(Config::default())
        }
    }

    #[test]
    fn test_access_token() {
        let client = MockClient {};
        let auth_response = AuthResponse::default();
        let config_provider = MockConfigProvider::default();
        let config = config_provider.get_config();
        let test_response = async_std::task::block_on(client.fetch_token(config, &auth_response));
        assert!(test_response.is_ok());
        let token = test_response.unwrap().access_token;
        assert!(!token.is_empty());
    }
}
