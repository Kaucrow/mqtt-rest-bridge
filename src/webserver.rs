use actix_web::{
    web, post, Responder, App, HttpServer, HttpResponse,
    dev::Server,
};
use rumqttc::{AsyncClient, QoS};

struct AppState {
    mqtt_client: AsyncClient,
}

pub struct WebServer {
    // Store the running Server instance, not the HttpServer builder
    server: Server,
}

impl WebServer {
    pub fn new(mqtt_client: AsyncClient) -> anyhow::Result<Self> {
        let data = web::Data::new(AppState { mqtt_client });

        let server = HttpServer::new(move || {
            App::new()
                .app_data(data.clone())
                .service(trigger_raspberry)
                .service(trigger_esp32)
                .service(stop_raspberry)
                .service(stop_esp32)
                .service(resume_raspberry)
                .service(resume_esp32)
        })
        .bind(("0.0.0.0", 3000))?
        .run();

        Ok(Self { server })
    }

    pub async fn run(self) -> std::io::Result<()> {
        self.server.await
    }
}

#[post("/play/raspberry")]
async fn trigger_raspberry(state: web::Data<AppState>) -> impl Responder {
    // Publish the command to the broker
    let result = state.mqtt_client
        .publish("commands/raspberry/play", QoS::AtLeastOnce, false, "true")
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Sent play command to Raspberry Pi"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to send command to Raspberry Pi"),
    }
}

#[post("/play/esp32")]
async fn trigger_esp32(state: web::Data<AppState>) -> impl Responder {
    // Publish the command to the broker
    let result = state.mqtt_client
        .publish("commands/esp32/play", QoS::AtLeastOnce, false, "true")
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Sent play command to ESP32"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to send command to ESP32"),
    }
}

#[post("/stop/raspberry")]
async fn stop_raspberry(state: web::Data<AppState>) -> impl Responder {
    // Publish the command to the broker
    let result = state.mqtt_client
        .publish("commands/raspberry/play", QoS::AtLeastOnce, false, "false")
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Sent stop command to Raspberry Pi"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to send command to Raspberry Pi"),
    }
}

#[post("/stop/esp32")]
async fn stop_esp32(state: web::Data<AppState>) -> impl Responder {
    // Publish the command to the broker
    let result = state.mqtt_client
        .publish("commands/esp32/play", QoS::AtLeastOnce, false, "false")
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Sent stop command to ESP32"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to send command to ESP32"),
    }
}

#[post("/resume/raspberry")]
async fn resume_raspberry(state: web::Data<AppState>) -> impl Responder {
    // Publish the command to the broker
    let result = state.mqtt_client
        .publish("commands/raspberry/play", QoS::AtLeastOnce, false, "")
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Sent resume command to Raspberry Pi"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to send command to Raspberry Pi"),
    }
}

#[post("/resume/esp32")]
async fn resume_esp32(state: web::Data<AppState>) -> impl Responder {
    // Publish the command to the broker
    let result = state.mqtt_client
        .publish("commands/esp32/play", QoS::AtLeastOnce, false, "")
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Sent resume command to ESP32"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to send command to ESP32"),
    }
}