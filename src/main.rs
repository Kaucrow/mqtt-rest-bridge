use mqtt_rest_bridge::{
    WebServer,
    prelude::*,
    config::get_config,
    telemetry,
    queries,
    db,
    broker,
    client,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config()?;

    // Init the tracing subscriber
    let (subscriber, _guard) = telemetry::get_subscriber(&config).await?;
    telemetry::init(subscriber);

    // Read DB queries and set global state
    queries::init()?;

    // Initialize database
    let db_path = &config.db.name;
    db::init(db_path)?;

    // Start MQTT broker in a background thread
    let (broker_host, broker_port) = broker::spawn_background_thread()?;

    // Give the broker half a second to bind to the port
    tokio::time::sleep(Duration::from_millis(500)).await;
    info!(
        "MQTT Broker running on {}. Available topics: {}.",
        format!("{}:{}", broker_host, broker_port).yellow(),
        "'sensors/esp32', 'sensors/raspberry', 'commands/esp32/play', 'commands/raspberry/play'".magenta()
    );

    // Start MQTT client
    let mqtt_client_host = "127.0.0.1";
    let mqtt_client = client::start_mqtt_client(
        mqtt_client_host,
        broker_port,
        config.db.name.clone()
    )
    .await
    .expect("MQTT Client failed to start.");
    info!(
        "MQTT Client listening for sensor data on {}...",
        format!("{}:{}", mqtt_client_host, broker_port).yellow()
    );

    // Start MQTT controller API
    info!(
        "MQTT Controller API starting on {}. View the docs at {}{}.",
        config.server.url().yellow(),
        format!("{}/", config.server.url()).cyan(),
        config.server.docs_endpoint.cyan().bold()
    );
    WebServer::new(&config, mqtt_client)?.run().await?;

    Ok(())
}