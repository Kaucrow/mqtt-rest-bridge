use crate::prelude::*;
use crate::{queries, QUERIES};
use rusqlite::{params, Connection};
use anyhow::anyhow;

#[derive(Clone)]
pub struct Db {
    path: String,
}

impl Db {
    pub fn init(db_path: &str) -> anyhow::Result<Self> {
        // Read DB queries and set global state
        queries::init()?;

        let conn = Connection::open(db_path)?;
        let queries = QUERIES.get().expect("Queries not initialized.");

        // Create tables
        conn.execute(&queries.presence.create, [])?;
        conn.execute(&queries.sensor_readings.create, [])?;
        conn.execute(&queries.play_cmd_log.create, [])?;

        Ok(Self { path: db_path.to_string() })
    }

    // --- Database manipulation ---

    pub fn update_status(&self, device: &str, payload: &str) -> anyhow::Result<()> {
        let conn = Connection::open(&self.path)?;
        let queries = QUERIES.get().expect("Queries not initialized.");

        conn.execute(
            &queries.sensor_readings.insert,
            params![device, payload]
        )?;

        Ok(())
    }

    pub fn insert_reading(&self, device: &str, payload: &str) -> anyhow::Result<()> {
        let conn = Connection::open(&self.path)?;
        let queries = QUERIES.get().expect("Queries not initialized.");

        match device {
            "esp32" => {
                let reading: ESP32SensorReading = serde_json::from_str(payload)?;
                conn.execute(
                    &queries.sensor_readings.insert,
                    params![
                        device,
                        reading.humidity,
                        reading.temperature,
                        reading.threshold,
                        reading.over_threshold,
                        rusqlite::types::Null
                    ],
                )?;
            }
            "raspberry" => {
                let reading: RaspberrySensorReading = serde_json::from_str(payload)?;
                conn.execute(
                    &queries.sensor_readings.insert,
                    params![
                        device,
                        reading.humidity,
                        reading.temperature,
                        reading.threshold,
                        rusqlite::types::Null,
                        reading.unit,
                    ],
                )?;
            }
            _ => return Err(anyhow!("No DB sensor reading insertion found for device {}.", device))
        }

        Ok(())
    }

    pub fn insert_play_cmd_log(&self, device: &str, payload: &str) -> anyhow::Result<()> {
        let conn = Connection::open(&self.path)?;
        let queries = QUERIES.get().expect("Queries not initialized.");

        conn.execute(
            &queries.play_cmd_log.insert,
            params![device, payload]
        )?;

        Ok(())
    }

}

#[derive(Deserialize)]
struct ESP32SensorReading {
    humidity: i32,
    temperature: i32,
    threshold: i32,
    over_threshold: bool,
}

#[derive(Deserialize)]
struct RaspberrySensorReading {
    humidity: i32,
    temperature: i32,
    threshold: i32,
    unit: String,
}