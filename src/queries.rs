use crate::prelude::*;
use crate::QUERIES;

#[derive(Deserialize, Debug)]
pub struct Queries {
    pub presence: Presence,
    pub sensor_readings: SensorReadings,
    pub play_cmd_log: PlayCmdLog,
}

#[derive(Deserialize, Debug)]
pub struct Presence {
    pub create: String,
    pub insert: String,
}

#[derive(Deserialize, Debug)]
pub struct SensorReadings {
    pub create: String,
    pub insert: String,
}

#[derive(Deserialize, Debug)]
pub struct PlayCmdLog {
    pub create: String,
    pub insert: String,
}

pub fn init() -> anyhow::Result<()> {
    let queries = get_queries()?;
    QUERIES.set(queries).expect("Failed to set global queries.");
    Ok(())
}

pub fn get_queries() -> Result<Queries, config::ConfigError> {
    let base_path = get_base_path();

    let queries_directory = base_path.join("queries");

    let filename = "queries.yaml";

    let settings = config::Config::builder()
        .add_source(config::File::from(
            queries_directory.join(filename),
        ))
        .build()?;

    settings.try_deserialize::<Queries>()
}