use crate::Db;
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use std::time::Duration;
use matchit::Router;
use tracing::{debug, error, warn};

type MqttHandlerFn = fn(&Db, &str, &str, &str) -> anyhow::Result<()>;

pub struct MqttClient {
    conn: AsyncClient,
}

impl MqttClient {
    pub async fn init(db: Db, host: &str, port: u16) -> anyhow::Result<Self> {
        let mut mqttoptions = MqttOptions::new("server-db-worker", host, port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        // Subscribe to all topics
        client.subscribe("presence/+/status", QoS::AtLeastOnce).await?;
        client.subscribe("sensors/+", QoS::AtLeastOnce).await?;
        client.subscribe("commands/+/play", QoS::AtLeastOnce).await?;

        let router = Self::init_router()?;

        // Spawn the event loop in a background task so it doesn't block
        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(notification) => {
                        if let Event::Incoming(Incoming::Publish(publish)) = notification {
                            let topic = publish.topic;
                            let payload = String::from_utf8_lossy(&publish.payload).to_string();

                            debug!("Received -> Topic: {}, Payload: {}", topic, payload);

                            let matched = match router.at(&topic) {
                                Ok(m) => m,
                                Err(_) => {
                                    warn!("No route defined for topic: {}", topic);
                                    continue;
                                }
                            };

                            let device = matched.params.get("device").unwrap_or("unknown");

                            let handler_fn = *matched.value;
                            if let Err(e) = handler_fn(&db, device, &topic, &payload) {
                                error!("Handler error for topic {}: {}", topic, e);
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
        Ok(Self { conn: client })
    }

    pub async fn publish(&self, topic: &str, qos: QoS, retain: bool, payload: &str) -> anyhow::Result<()> {
        self.conn
            .publish(topic, qos, retain, payload)
            .await?;

        Ok(())
    }

    // --- Topic handlers router ---

    fn init_router() -> anyhow::Result<Router<MqttHandlerFn>> {
        let mut router = Router::new();

        router.insert("presence/{device}/status", Self::presence_handler as MqttHandlerFn)?;
        router.insert("sensors/{device}", Self::sensors_handler as MqttHandlerFn)?;
        router.insert("commands/{device}/play", Self::play_handler as MqttHandlerFn)?;

        Ok(router)
    }

    // --- Topic handlers ---

    fn presence_handler(db: &Db, device: &str, _topic: &str, payload: &str) -> anyhow::Result<()> {
        db.update_status(device, payload)?;
        Ok(())
    }

    fn sensors_handler(db: &Db, device: &str, _topic: &str, payload: &str) -> anyhow::Result<()> {
        db.insert_reading(device, payload)?;
        Ok(())
    }

    fn play_handler(db: &Db, device: &str, _topic: &str, payload: &str) -> anyhow::Result<()> {
        db.insert_play_cmd_log(device, payload)?;
        Ok(())
    }
}