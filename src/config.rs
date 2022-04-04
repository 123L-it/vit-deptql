use dotenv::dotenv;
use url::Url;

pub struct AppConfig {
    pub user_api: Url,
}

/// Get server application configuration
/// from .env file variables.
pub fn get_config() -> AppConfig {
    dotenv().expect("Failed to read .env file, copy .env.example to .env");
    let user_api_env = format!("{}{}", dotenv::var("USER_API_URL").unwrap(), '/');

    AppConfig {
        user_api: Url::parse(&user_api_env).unwrap(),
    }
}
