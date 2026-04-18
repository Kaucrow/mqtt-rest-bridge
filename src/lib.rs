pub mod prelude;
pub mod config;
pub mod queries;
pub mod telemetry;
pub mod db;
pub mod broker;
pub mod client;
mod webserver;

pub use webserver::WebServer;
pub use db::Db;
pub use broker::MqttBroker;
pub use client::MqttClient;

use std::sync::OnceLock;
use queries::Queries;

static QUERIES: OnceLock<Queries> = OnceLock::new();