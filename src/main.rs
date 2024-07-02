//! Run with `cargo run --example hello_world` command.
//!
//! To connect through browser, navigate to "http://localhost:3000" url.

use axum::{
    routing::{get, post}, Router, extract::Path, Json, http::StatusCode, body::Bytes
};
use std::net::SocketAddr;
use std::collections::HashMap;
use serde_json::json;
use std::sync::{Arc, Mutex};
use axum::extract::State;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool
}

struct DataBase {
    map : HashMap<String, Movie>,
}

impl DataBase {
    
    fn new() -> Self {
        let mut map = HashMap::new();
        Self {
            map,
        }
    }
}

#[tokio::main]
async fn main() {

    let db = Arc::new(Mutex::new(DataBase::new()));

    let app = Router::new()
    .route("/movie/:id", get(get_movie))
    .route("/movie", post(post_movie))
    //.route("/movie", post(|| async { "POST" }))
    .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();
    axum::serve(listener, app).await.unwrap();

}

async fn get_movie(
    Path(id): Path<String>,
    State(db): State<Arc<Mutex<DataBase>>>,
) -> impl IntoResponse {
    let db = db.lock().unwrap();
    if let Some(movie) = db.map.get(&id) {
        match serde_json::to_string(&movie) {
            Ok(movie_json) => (StatusCode::OK, movie_json),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to serialize movie".to_string()),
        }
    } else {
        (StatusCode::NOT_FOUND, "Movie not found".to_string())
    }
}

async fn post_movie(
    State(db): State<Arc<Mutex<DataBase>>>,
    Json(movie): Json<Movie>,
) -> (StatusCode, Json<Movie>) {
    let mut db = db.lock().unwrap();
    let movie_clone = movie.clone();
    db.map.insert(movie.id.clone(), movie);
    (StatusCode::CREATED, Json(movie_clone))
}

//     // Create Axum server with the following endpoints:
//     // 1. GET /movie/{id} - This should return back a movie given the id
//     // 2. POST /movie - this should save movie in a DB (HashMap<String, Movie>). This movie will be sent
//     // via a JSON payload. 
    
//     // As a bonus: implement a caching layer so we don't need to make expensive "DB" lookups, etc.
    

