//! Run with `cargo run --example hello_world` command.
//!
//! To connect through browser, navigate to "http://localhost:3000" url.

use axum::{routing::{get, post}, Router, extract::Path, Json, http::StatusCode};
use std::net::SocketAddr;
use std::collections::HashMap;
use serde_json::json;
use std::sync::{Arc, Mutex};
use axum::extract::State;
use axum::response::IntoResponse;

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
        let map = HashMap::new();
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
    .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn get_movie(
    Path(id): Path<String>,
    State(db): State<Arc<Mutex<DataBase>>>,
) -> impl IntoResponse {
    let db = db.lock().unwrap();
    if let Some(movie) = db.map.get(&id) {
        (StatusCode::OK, Json(movie.clone()))
    }
}

async fn post_movie(
    Json(movie): Json<Movie>,
    State(db): State<Arc<Mutex<DataBase>>>,
) -> impl IntoResponse {
    let mut db = db.lock().unwrap();
    db.map.insert(movie.id.clone(), movie);
    (StatusCode::CREATED, Json(json!({ "status": "Movie added" })))
}

//     // Create Axum server with the following endpoints:
//     // 1. GET /movie/{id} - This should return back a movie given the id
//     // 2. POST /movie - this should save movie in a DB (HashMap<String, Movie>). This movie will be sent
//     // via a JSON payload. 
    
//     // As a bonus: implement a caching layer so we don't need to make expensive "DB" lookups, etc.
    

