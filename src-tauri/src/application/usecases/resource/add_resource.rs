use std::sync::Arc;
use crate::domain::repositories::resource_repository::{ResourceRepository, RepositoryError};
use crate::domain::entities::resource::Resource;

pub struct AddResourceUseCase {
    repo: Arc<dyn ResourceRepository>,
}

impl AddResourceUseCase {
    pub fn new(repo: Arc<dyn ResourceRepository>) -> Self {
        Self { repo }
    }
    
    pub async fn execute(&self, resource: Resource) -> Result<(), RepositoryError> {
        self.repo.create(&resource).await
    }
}

