use crate::models::Task;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
pub struct TaskRepository {
    tasks: RwLock<HashMap<Uuid, Task>>,
}

impl TaskRepository {
    pub fn new() -> Self {
        Self {
            tasks: RwLock::new(HashMap::new()),
        }
    }

    pub fn create(&self, task: Task) -> Result<Task, String> {
        let mut tasks = self
            .tasks
            .write()
            .map_err(|_| "Failed to acquire write lock")?;
        let id = task.id;
        tasks.insert(id, task.clone());
        Ok(task)
    }

    pub fn find_by_id(&self, id: &Uuid) -> Result<Option<Task>, String> {
        let tasks = self
            .tasks
            .read()
            .map_err(|_| "Failed to acquire read lock")?;
        Ok(tasks.get(id).cloned())
    }

    pub fn find_all(&self) -> Result<Vec<Task>, String> {
        let tasks = self
            .tasks
            .read()
            .map_err(|_| "Failed to acquire read lock")?;
        Ok(tasks.values().cloned().collect())
    }

    pub fn find_by_project_id(&self, project_id: &Uuid) -> Result<Vec<Task>, String> {
        let tasks = self
            .tasks
            .read()
            .map_err(|_| "Failed to acquire read lock")?;
        Ok(tasks
            .values()
            .filter(|task| task.project_id == *project_id)
            .cloned()
            .collect())
    }

    pub fn update(&self, id: &Uuid, updated_task: Task) -> Result<Option<Task>, String> {
        let mut tasks = self
            .tasks
            .write()
            .map_err(|_| "Failed to acquire write lock")?;
        if tasks.contains_key(id) {
            tasks.insert(*id, updated_task.clone());
            Ok(Some(updated_task))
        } else {
            Ok(None)
        }
    }

    pub fn delete(&self, id: &Uuid) -> Result<bool, String> {
        let mut tasks = self
            .tasks
            .write()
            .map_err(|_| "Failed to acquire write lock")?;
        Ok(tasks.remove(id).is_some())
    }

    pub fn delete_by_project_id(&self, project_id: &Uuid) -> Result<usize, String> {
        let mut tasks = self
            .tasks
            .write()
            .map_err(|_| "Failed to acquire write lock")?;
        let ids_to_remove: Vec<Uuid> = tasks
            .values()
            .filter(|task| task.project_id == *project_id)
            .map(|task| task.id)
            .collect();

        let count = ids_to_remove.len();
        for id in ids_to_remove {
            tasks.remove(&id);
        }
        Ok(count)
    }
}

impl Default for TaskRepository {
    fn default() -> Self {
        Self::new()
    }
}
