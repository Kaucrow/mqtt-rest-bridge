use crate::prelude::*;
use crate::{
    MqttClient,
    config::Config
};
use actix_web::{
    web, post, Responder, App, HttpServer, HttpResponse,
    dev::Server,
};
use rumqttc::QoS;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

struct AppState {
    mqtt_client: MqttClient,
}

#[derive(OpenApi)]
#[openapi(
    paths(handle_mqtt_command),
    info(description = "MQTT Controller API")
)]
struct ApiDoc;

pub struct WebServer {
    server: Server,
}

impl WebServer {
    pub fn new(config: &Config, mqtt_client: MqttClient) -> anyhow::Result<Self> {
        let data = web::Data::new(AppState { mqtt_client });
        let openapi = ApiDoc::openapi();

        let docs_endpoint = config.api.docs_endpoint.clone();
        let server = HttpServer::new(move || {
            App::new()
                .app_data(data.clone())
                .service(handle_mqtt_command)
                .service(Scalar::with_url(format!("/{}", docs_endpoint), openapi.clone()))
        })
        .bind((config.api.host.clone(), config.api.port))?
        .run();

        Ok(Self { server })
    }

    pub async fn run(self) -> std::io::Result<()> {
        self.server.await
    }
}

#[utoipa::path(
    post,
    path = "/{action}/{device}",
    responses(
        (status = 200, description = "Command sent successfully via MQTT"),
        (status = 400, description = "Unknown action or device requested"),
        (status = 500, description = "Internal server error connecting to MQTT broker")
    ),
    params(
        ("action" = String, Path, description = "The action to perform. Valid options: play, stop, resume"),
        ("device" = String, Path, description = "The target device. Valid options: raspberry, esp32")
    )
)]
#[post("/{action}/{device}")]
async fn handle_mqtt_command(
    path: web::Path<(String, String)>, 
    state: web::Data<AppState>
) -> impl Responder {
    let (action, device) = path.into_inner();

    let topic = match device.as_str() {
        "raspberry" => "commands/raspberry/play",
        "esp32" => "commands/esp32/play",
        _ => {
            error!("Invalid device requested: {}", device);
            return HttpResponse::BadRequest()
                .content_type("text/plain")
                .body("Unknown device");
        }
    };

    let payload = match action.as_str() {
        "play" => "true",
        "stop" => "false",
        "resume" => "",
        _ => {
            error!("Invalid action requested: {}", action);
            return HttpResponse::BadRequest()
                .content_type("text/plain")
                .body("Unknown action");
        }
    };

    let result = state.mqtt_client
        .publish(topic, QoS::AtLeastOnce, false, payload)
        .await;

    match result {
        Ok(_) => {
            info!("Successfully sent '{}' command to {}", action, device);
            HttpResponse::Ok()
                .content_type("text/plain")
                .body(format!("Sent '{}' command to {}", action, device))
        }
        Err(e) => {
            error!("Failed to send '{}' command to {}: {:?}", action, device, e);
            HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body(format!("Failed to send command to {}", device))
        }
    }
}