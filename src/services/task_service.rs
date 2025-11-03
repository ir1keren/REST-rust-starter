use crate::models::{Task, TaskCreate, TaskUpdate};
use crate::repositories::{ProjectRepository, TaskRepository};
use crate::views::ApiError;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TaskService {
    task_repository: Arc<TaskRepository>,
    project_repository: Arc<ProjectRepository>,
}

impl TaskService {
    pub fn new(
        task_repository: Arc<TaskRepository>,
        project_repository: Arc<ProjectRepository>,
    ) -> Self {
        Self {
            task_repository,
            project_repository,
        }
    }

    pub async fn create_task(&self, create_data: TaskCreate) -> Result<Task, ApiError> {
        // Validation
        if create_data.title.trim().is_empty() {
            return Err(ApiError::validation_error("Task title cannot be empty"));
        }

        if create_data.title.len() > 200 {
            return Err(ApiError::validation_error(
                "Task title cannot exceed 200 characters",
            ));
        }

        if let Some(ref description) = create_data.description {
            if description.len() > 1000 {
                return Err(ApiError::validation_error(
                    "Task description cannot exceed 1000 characters",
                ));
            }
        }

        // Verify project exists
        self.project_repository
            .find_by_id(&create_data.project_id)
            .map_err(|e| ApiError::repository_error(&e))?
            .ok_or_else(|| ApiError::not_found("Project"))?;

        let task = Task::new(
            create_data.project_id,
            create_data.title.trim().to_string(),
            create_data.description,
        );

        self.task_repository
            .create(task)
            .map_err(|e| ApiError::repository_error(&e))
    }

    pub async fn get_task(&self, id: Uuid) -> Result<Task, ApiError> {
        self.task_repository
            .find_by_id(&id)
            .map_err(|e| ApiError::repository_error(&e))?
            .ok_or_else(|| ApiError::not_found("Task"))
    }

    pub async fn list_tasks(&self) -> Result<Vec<Task>, ApiError> {
        self.task_repository
            .find_all()
            .map_err(|e| ApiError::repository_error(&e))
    }

    pub async fn list_tasks_by_project(&self, project_id: Uuid) -> Result<Vec<Task>, ApiError> {
        // Verify project exists
        self.project_repository
            .find_by_id(&project_id)
            .map_err(|e| ApiError::repository_error(&e))?
            .ok_or_else(|| ApiError::not_found("Project"))?;

        self.task_repository
            .find_by_project_id(&project_id)
            .map_err(|e| ApiError::repository_error(&e))
    }

    pub async fn update_task(&self, id: Uuid, update_data: TaskUpdate) -> Result<Task, ApiError> {
        // Validation
        if let Some(ref title) = update_data.title {
            if title.trim().is_empty() {
                return Err(ApiError::validation_error("Task title cannot be empty"));
            }
            if title.len() > 200 {
                return Err(ApiError::validation_error(
                    "Task title cannot exceed 200 characters",
                ));
            }
        }

        if let Some(ref description) = update_data.description {
            if description.len() > 1000 {
                return Err(ApiError::validation_error(
                    "Task description cannot exceed 1000 characters",
                ));
            }
        }

        // Get existing task
        let mut task = self
            .task_repository
            .find_by_id(&id)
            .map_err(|e| ApiError::repository_error(&e))?
            .ok_or_else(|| ApiError::not_found("Task"))?;

        // Apply updates
        task.update(update_data);

        // Save updated task
        self.task_repository
            .update(&id, task)
            .map_err(|e| ApiError::repository_error(&e))?
            .ok_or_else(|| ApiError::not_found("Task"))
    }

    pub async fn delete_task(&self, id: Uuid) -> Result<(), ApiError> {
        let deleted = self
            .task_repository
            .delete(&id)
            .map_err(|e| ApiError::repository_error(&e))?;

        if deleted {
            Ok(())
        } else {
            Err(ApiError::not_found("Task"))
        }
    }

    pub async fn delete_tasks_by_project(&self, project_id: Uuid) -> Result<usize, ApiError> {
        self.task_repository
            .delete_by_project_id(&project_id)
            .map_err(|e| ApiError::repository_error(&e))
    }
}
