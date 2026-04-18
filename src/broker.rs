use crate::prelude::*;
use crate::config::get_broker_config;
use rumqttd::{
    Broker as RumqttdBroker,
    Config as RumqttdConfig,
};
use std::thread;

pub struct MqttBroker {
    pub host: String,
    pub port: u16,
}

impl MqttBroker {
    pub fn init() -> anyhow::Result<Self> {
        let broker_config = get_broker_config().expect("Failed to get broker config.");
        let (host, port) = Self::addr_from_config(&broker_config)?;
        thread::spawn(|| {
            let mut broker = RumqttdBroker::new(broker_config);

            // Blocking call
            broker.start().expect("MQTT Broker crashed.");
        });

        Ok(Self { host, port })
    }

    fn addr_from_config(broker_config: &RumqttdConfig) -> anyhow::Result<(String, u16)> {
        let listen_addr = broker_config
            .v4
            .as_ref()
            .and_then(|servers| servers.values().next())
            .map(|server_settings| server_settings.listen.to_string())
            // Fallback to v5 if v4 isn't configured, or "unknown" if neither are
            .or_else(|| {
                broker_config
                    .v5
                    .as_ref()
                    .and_then(|servers| servers.values().next())
                    .map(|server_settings| server_settings.listen.to_string())
            })
            .unwrap_or_else(|| "unknown address".to_string());

        let (host, port) = listen_addr.split_once(':').ok_or(anyhow!(""))?;

        Ok((host.to_string(), port.parse::<u16>()?))
    }
}