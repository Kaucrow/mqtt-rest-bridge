use crate::prelude::*;
use crate::config::get_broker_config;
use rumqttd::{
    Broker,
    Config as RumqttdConfig
};
use std::thread;

pub fn spawn_background_thread() -> anyhow::Result<(String, u16)> {
    let broker_config = get_broker_config().expect("Failed to get broker config.");
    let (host, port) = get_broker_addr(&broker_config)?;
    thread::spawn(|| {
        let mut broker = Broker::new(broker_config);

        // Blocking call
        broker.start().expect("MQTT Broker crashed");
    });

    Ok((host, port))
}

fn get_broker_addr(broker_config: &RumqttdConfig) -> anyhow::Result<(String, u16)> {
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