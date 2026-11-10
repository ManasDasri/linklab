//A Read only URL-shortner
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::get,
};
use std::{collections::HashMap, sync::Arc};

#[tokio::main]
async fn main() {
    let mut links = HashMap::new();

    links.insert(String::from("abc123"), String::from("https://example.com"));

    links.insert(String::from("rust"), String::from("https://rust-lang.org"));

    links.insert(String::from("github"), String::from("https://github.com"));
    let shared_links = Arc::new(links); //creating a common ownership based hashmap

    let app: Router = Router::new()
        .route("/", get(home))
        .route("/health", get(health))
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
    State(links): State<Arc<HashMap<String, String>>>,
) -> Result<Redirect, StatusCode> {
    match links.get(&code) {
        Some(url) => Ok(Redirect::temporary(url)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
