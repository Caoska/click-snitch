use axum::{http::StatusCode, routing::get, routing::post, Json, Router};
use tokio::net::TcpListener;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[derive(Serialize)]
struct Event {
    event: String,
    text: String,
    tag: String,
    path: String,
    timestamp: String
}

#[tokio::main]
async fn main() {
    // 1. Define your routes and handlers
    let app = Router::new()
    .route("/", get(default_handler))
    .route("/collect", post(collect_handler))
    .route("/events", get(events_handler));

    // 2. Create a TCP listener on a specific address and port
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    // 3. Start the server
    axum::serve(listener, app).await.unwrap();
}

// Handler function that returns a simple string response
async fn default_handler() -> &'static str {
    "Hello, World!"
}

// Handler function that returns a simple string response
async fn collect_handler(Json(payload): Json<Event>,) -> StatusCode {
    let mut line = serde_json::to_string(&payload).expect("Failed to serialize");
    println!("payload: {}", line);
    line.push('\n');
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("data.jsonl")
        .await
        .expect("Could not open or create file");
    file.write_all(line.as_bytes()).await.expect("Failed to write to file");

    StatusCode::OK
}

// Handler function that returns a simple string response
async fn events_handler() -> Json<Vec<Event>> {
    let file = File::open("data.jsonl").await.expect("Failed to open file");
    let mut reader = BufReader::new(file).lines();
    let mut results = Vec::new();

    while let Some(line) = reader.next_line().await.expect("Failed to read line") {
        if let Ok(json_val) = serde_json::from_str::<Event>(&line) {
            results.push(json_val);
        }
    }

    Json(results)
}