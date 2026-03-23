use axum::{
    http::StatusCode,
    routing::{get, post},
    Json,
    Router
};
use tokio::net::TcpListener;
use tokio::fs::OpenOptions;
use tokio::io::{
    AsyncWriteExt,
    AsyncBufReadExt,
    BufReader
};
use tower_http::cors::{
    Any,
    CorsLayer
};
use serde::{
    Deserialize,
    Serialize
};
use std::collections::HashMap;

#[derive(Deserialize)]
#[derive(Serialize)]
struct Event {
    sessionId: String,
    event: String,
    text: Option<String>,
    tag: String,
    id: Option<String>,
    classes: Option<String>,
    path: String,
    timestamp: String
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(default_handler))
        .route("/collect", post(collect_handler))
        .route("/events", get(events_handler));
    
    let app = app.layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    );

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

// Handler function that returns a simple string response
async fn default_handler() -> &'static str {
    "Hello, World!"
}

// Handler function that returns a simple string response
async fn collect_handler(Json(payload): Json<Event>,) -> StatusCode {
    let mut line = serde_json::to_string(&payload).expect("Failed to serialize");

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
async fn events_handler() -> Json<HashMap<String, Vec<Event>>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("data.jsonl")
        .await
        .expect("Failed to open file");
    let mut reader = BufReader::new(file).lines();
    let mut sessions: HashMap<String, Vec<Event>> = HashMap::new();

    while let Some(line) = reader.next_line().await.expect("Failed to read line") {
        if let Ok(event) = serde_json::from_str::<Event>(&line) {
            sessions
                .entry(event.sessionId.clone())
                .or_insert_with(Vec::new)
                .push(event);
        }
    }

    for events in sessions.values_mut() {
        events.sort_by_key(|e| e.timestamp.clone());
    }

    Json(sessions)
}