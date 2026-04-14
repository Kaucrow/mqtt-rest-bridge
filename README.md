# MQTT Broker & REST API SCADA bridge

This project provides the backend infrastructure for a demo SCADA system. It bridges two microcontrollers (a Raspberry Pi and an ESP32) with a real-time dashboard and an SQLite database for data logs. The backend consists of a Rust-based MQTT broker for real-time telemetry/commands and a REST API for dashboard integration.

## System Architecture

* **REST API:** Built with Rust (Actix-Web) to handle HTTP commands from the real-time dashboard. OpenAPI documentation is included and visualized with Scalar.
* **MQTT Broker:** Facilitates bi-directional communication between the microcontrollers and the backend.
* **Microcontrollers:** A Raspberry Pi and an ESP32 equipped with sensors and buzzers.
* **Database:** SQLite is used to persist sensor telemetry and system state logs.
* **Real-Time Dashboard:** A frontend interface (not included in this repo) that monitors sensor values and issues manual override commands via the REST API.

## Core System Logic & Behavior

The system features a response mechanism between the two microcontrollers based on their sensor readings and the current command state.

By default (or when "resumed"), the microcontrollers operate in a **Cross-Linked Mode**:
* The **ESP32** plays a melody based on the sensor values of the **Raspberry Pi**.
* The **Raspberry Pi** plays a melody based on the sensor values of the **ESP32**.

This automatic behavior can be manually overridden via the REST API/Dashboard. The system responds to three specific state commands:
* `resume` (Payload: `""`): Microcontrollers return to listening to each other's sensors.
* `play` (Payload: `"true"`): Manual override **ON**. The target device plays its melody continuously, disregarding sensor values entirely.
* `stop` (Payload: `"false"`): Manual override **OFF**. The target device stops playing entirely, disregarding sensor values.

## MQTT Topic Structure

### Telemetry (Sensors)
Devices publish their current sensor readings to these topics:
* `sensors/raspberry`
* `sensors/esp32`

### Commands (Actuators)
Devices subscribe to these topics to listen for override commands from the API:
* `commands/raspberry/play`
* `commands/esp32/play`

## REST API Documentation

### Interactive Docs (Scalar)
When the project is running, you can access the full interactive OpenAPI documentation powered by **Scalar**. Simply navigate to the configured docs route, `http://localhost:8080/docs` in your browser.

## Getting Started

### Prerequisites
* Rust toolchain
* SQLite3

### Usage
1. Clone this repository.
2. Compile and run the application with `cargo run`.