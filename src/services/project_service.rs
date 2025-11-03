use std::sync::Arc;
use uuid::Uuid;
use crate::models::{Project, ProjectCreate, ProjectUpdate};
use crate::repositories::ProjectRepository;
use crate::views::ApiError;

#[derive(Debug, Clone)]
pub struct ProjectService {
    repository: Arc<ProjectRepository>,
}

impl ProjectService {
    pub fn new(repository: Arc<ProjectRepository>) -> Self {
        Self { repository }
    }
    
    pub async fn create_project(&self, create_data: ProjectCreate) -> Result<Project, ApiError> {
        // Validation
        if create_data.name.trim().is_empty() {
            return Err(ApiError::validation_error("Project name cannot be empty"));
        }
        
        if create_data.name.len() > 200 {
            return Err(ApiError::validation_error("Project name cannot exceed 200 characters"));
        }
        
        if let Some(ref description) = create_data.description {
            if description.len() > 1000 {
                return Err(ApiError::validation_error("Project description cannot exceed 1000 characters"));
            }
        }
        
        let project = Project::new(create_data.name.trim().to_string(), create_data.description);
        
        self.repository
            .create(project)
            .map_err(|e| ApiError::repository_error(&e))
    }
    
    pub async fn get_project(&self, id: Uuid) -> Result<Project, ApiError> {
        self.repository
            .find_by_id(&id)
            .map_err(|e| ApiError::repository_error(&e))?
            .ok_or_else(|| ApiError::not_found("Project"))
    }
    
    pub async fn list_projects(&self) -> Result<Vec<Project>, ApiError> {
        self.repository
            .find_all()
            .map_err(|e| ApiError::repository_error(&e))
    }
    
    pub async fn update_project(&self, id: Uuid, update_data: ProjectUpdate) -> Result<Project, ApiError> {
        // Validation
        if let Some(ref name) = update_data.name {
            if name.trim().is_empty() {
                return Err(ApiError::validation_error("Project name cannot be empty"));
            }
            if name.len() > 200 {
                return Err(ApiError::validation_error("Project name cannot exceed 200 characters"));
            }
        }
        
        if let Some(ref description) = update_data.description {
            if description.len() > 1000 {
                return Err(ApiError::validation_error("Project description cannot exceed 1000 characters"));
            }
        }
        
        // Get existing project
        let mut project = self.repository
            .find_by_id(&id)
            .map_err(|e| ApiError::repository_error(&e))?
            .ok_or_else(|| ApiError::not_found("Project"))?;
        
        // Apply updates
        project.update(update_data);
        
        // Save updated project
        self.repository
            .update(&id, project)
            .map_err(|e| ApiError::repository_error(&e))?
            .ok_or_else(|| ApiError::not_found("Project"))
    }
    
    pub async fn delete_project(&self, id: Uuid) -> Result<(), ApiError> {
        let deleted = self.repository
            .delete(&id)
            .map_err(|e| ApiError::repository_error(&e))?;
        
        if deleted {
            Ok(())
        } else {
            Err(ApiError::not_found("Project"))
        }
    }
}