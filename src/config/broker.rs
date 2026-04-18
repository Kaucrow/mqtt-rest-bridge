use crate::prelude::*;
use rumqttd::Config as RumqttdConfig;

pub fn get_broker_config() -> Result<RumqttdConfig, config::ConfigError> {
    let base_path = get_base_path();

    let environment: String = std::env::var("RAILWAY_ENVIRONMENT_NAME")
        .unwrap_or_else(|_| "development".into());

    let config_directory = base_path.join(format!("config/{}", environment));

    let filename = "broker.toml";

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

    settings.try_deserialize::<RumqttdConfig>()
}