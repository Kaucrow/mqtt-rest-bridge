use crate::prelude::*;
use strum::Display;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub debug: bool,
    pub server: ServerConfig,
    #[serde(rename = "database")]
    pub db: DbConfig,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub protocol: ServerProtocol,
    pub docs_endpoint: String,
}

impl ServerConfig {
    pub fn url(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }
}

#[derive(Deserialize, Debug, Display)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ServerProtocol {
    Http,
    Https,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DbConfig {
    pub name: String,
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    let base_path = get_base_path();

    let environment: String = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".into());

    let config_directory = base_path.join(format!("config/{}", environment));

    let filename = "base.toml";

    let settings = config::Config::builder()
        .add_source(config::File::from(
            config_directory.join(filename),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Config>()
}