use std::sync::Arc;
use crate::domain::repositories::knowledge_base_repository::{KnowledgeBaseRepository, KnowledgeBaseRepositoryError};

pub struct ListKnowledgeBasesUseCase {
    repo: Arc<dyn KnowledgeBaseRepository>,
}

impl ListKnowledgeBasesUseCase {
    pub fn new(repo: Arc<dyn KnowledgeBaseRepository>) -> Self {
        Self { repo }
    }
    
    pub async fn execute(&self) -> Result<Vec<crate::domain::entities::knowledge_base::KnowledgeBase>, KnowledgeBaseRepositoryError> {
        self.repo.list().await
    }
}

