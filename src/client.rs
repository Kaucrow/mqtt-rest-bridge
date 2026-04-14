use crate::db;
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use std::time::Duration;
use tracing::{debug, error};

pub async fn start_mqtt_client(host: &str, port: u16, db_path: String) -> anyhow::Result<AsyncClient> {
    let mut mqttoptions = MqttOptions::new("server-db-worker", host, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // Subscribe to all topics under 'sensors/'
    client.subscribe("sensors/#", QoS::AtLeastOnce).await?;

    // Spawn the event loop in a background task so it doesn't block
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(notification) => {
                    if let Event::Incoming(Incoming::Publish(publish)) = notification {
                        let topic = publish.topic;
                        let payload = String::from_utf8_lossy(&publish.payload).to_string();

                        debug!("Received -> Topic: {}, Payload: {}", topic, payload);

                        if let Err(e) = db::insert_reading(&db_path, &topic, &payload) {
                            error!("Database error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("MQTT connection error: {:?}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    });

    // Return the client so the web server can use it to publish commands
    Ok(client)
}