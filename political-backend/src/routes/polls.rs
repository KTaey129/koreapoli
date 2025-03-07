use axum::{Json, Router, routing::{get, post}};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Poll {
    id: Uuid,
    question: String,
    options: Vec<String>,
    votes: Vec<u32>,
}
// global memory storage (temporary usage rather than DB)
type SharedState = Arc<Mutex<Vec<Poll>>>;
pub fn poll_routes() -> Router {
    let state = Arc::new(Mutex::new(vec![]));
    Router::new()
        .route("/polls", post(create_poll))
        .route("/polls", get(get_polls))
        .with_state(state)
}

// generate poll
async fn create_poll(
    state: axum::extract::State<SharedState>,
    Json(payload): Json<PollRequest>,
) -> Json<Poll> {
    let mut polls = state.lock().unwrap();

    let new_poll = Poll {
        id: Uuid::new_v4(),
        question: payload.question,
        options: payload.options.clone(),
        votes: vec![0; payload.options.len()],
    };
    polls.push(new_poll.clone());
    Json(new_poll)
}

// Retrieve poll
async fn get_polls(
    state: axum::extract::State<SharedState>,
) -> Json<Vec<Poll>> {
    let polls = state.lock().unwrap();
    Json(polls.clone())
}

// Request body struct
#[derive(Debug, Deserialize)]
struct PollRequest {
    question: String,
    options: Vec<String>,
}