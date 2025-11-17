use std::sync::Arc;
use crate::domain::repositories::resource_repository::{ResourceRepository, RepositoryError};

pub struct ListResourcesUseCase {
    repo: Arc<dyn ResourceRepository>,
}

impl ListResourcesUseCase {
    pub fn new(repo: Arc<dyn ResourceRepository>) -> Self {
        Self { repo }
    }
    
    pub async fn execute(&self) -> Result<Vec<crate::domain::entities::resource::Resource>, RepositoryError> {
        self.repo.list().await
    }
}

