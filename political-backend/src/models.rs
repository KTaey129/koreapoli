use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    pub id: Uuid,
    pub question: String,
    pub options: Vec<String>,
    pub votes: Vec<u32>,
}