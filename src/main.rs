//A Read only URL-shortner
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::get,
    {Json, routing::post},
};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

//struct creation
#[derive(Deserialize)]
struct CreateLink {
    code: String,
    url: String,
}

#[tokio::main]
async fn main() {
    let mut links = HashMap::new();

    links.insert(String::from("abc123"), String::from("https://example.com"));

    links.insert(String::from("rust"), String::from("https://rust-lang.org"));

    links.insert(String::from("github"), String::from("https://github.com"));
    let shared_links = Arc::new(RwLock::new(links)); //creating a common ownership based hashmap

    let app: Router = Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/links", post(create_link))
        .route("/{code}", get(show_code))
        .with_state(shared_links);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("LinkLab is running! at port: 3000");
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> String {
    String::from("Hello from LinkLab")
}
async fn health() -> String {
    String::from("LinkLab Server is Healthy!")
}

async fn show_code(
    Path(code): Path<String>,
    State(links): State<Arc<RwLock<HashMap<String, String>>>>,
) -> Result<Redirect, StatusCode> {
    let map = links.read().await;
    println!("Read Lock Required");

    match map.get(&code) {
        Some(url) => Ok(Redirect::temporary(url)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_link(
    State(links): State<Arc<RwLock<HashMap<String, String>>>>,
    Json(payload): Json<CreateLink>,
) -> StatusCode {
    if payload.code.is_empty()
        || payload.code.contains('/')
        || payload.code == "health"
        || payload.code == "links"
        || !(payload.url.starts_with("https://") || payload.url.starts_with("http://"))
    {
        return StatusCode::BAD_REQUEST;
    }

    let mut map = links.write().await;
    println!("Write Lock Required");

    if map.contains_key(&payload.code) {
        return StatusCode::CONFLICT;
    }

    map.insert(payload.code, payload.url);

    StatusCode::CREATED
}
