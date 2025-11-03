use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct Task {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct TaskCreate {
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct TaskUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub done: Option<bool>,
}

impl Task {
    pub fn new(project_id: Uuid, title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            project_id,
            title,
            description,
            done: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, update: TaskUpdate) {
        if let Some(title) = update.title {
            self.title = title;
        }
        if update.description.is_some() {
            self.description = update.description;
        }
        if let Some(done) = update.done {
            self.done = done;
        }
        self.updated_at = Utc::now();
    }
}
