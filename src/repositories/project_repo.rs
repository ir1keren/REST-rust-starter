use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;
use crate::models::Project;

#[derive(Debug)]
pub struct ProjectRepository {
    projects: RwLock<HashMap<Uuid, Project>>,
}

impl ProjectRepository {
    pub fn new() -> Self {
        Self {
            projects: RwLock::new(HashMap::new()),
        }
    }
    
    pub fn create(&self, project: Project) -> Result<Project, String> {
        let mut projects = self.projects.write().map_err(|_| "Failed to acquire write lock")?;
        let id = project.id;
        projects.insert(id, project.clone());
        Ok(project)
    }
    
    pub fn find_by_id(&self, id: &Uuid) -> Result<Option<Project>, String> {
        let projects = self.projects.read().map_err(|_| "Failed to acquire read lock")?;
        Ok(projects.get(id).cloned())
    }
    
    pub fn find_all(&self) -> Result<Vec<Project>, String> {
        let projects = self.projects.read().map_err(|_| "Failed to acquire read lock")?;
        Ok(projects.values().cloned().collect())
    }
    
    pub fn update(&self, id: &Uuid, updated_project: Project) -> Result<Option<Project>, String> {
        let mut projects = self.projects.write().map_err(|_| "Failed to acquire write lock")?;
        if projects.contains_key(id) {
            projects.insert(*id, updated_project.clone());
            Ok(Some(updated_project))
        } else {
            Ok(None)
        }
    }
    
    pub fn delete(&self, id: &Uuid) -> Result<bool, String> {
        let mut projects = self.projects.write().map_err(|_| "Failed to acquire write lock")?;
        Ok(projects.remove(id).is_some())
    }
}

impl Default for ProjectRepository {
    fn default() -> Self {
        Self::new()
    }
}