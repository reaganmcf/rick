use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: usize,
    pub title: String,
    pub desc: Option<String>,
}
