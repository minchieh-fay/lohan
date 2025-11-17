use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl KnowledgeBase {
    pub fn new(id: String, name: String, base_path: &PathBuf) -> Self {
        let path = base_path.join(&id);
        let now = Utc::now();
        Self {
            id,
            name,
            path,
            description: None,
            created_at: now,
            updated_at: now,
        }
    }
}

