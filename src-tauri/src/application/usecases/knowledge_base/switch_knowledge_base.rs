use std::sync::Arc;
use crate::domain::repositories::knowledge_base_repository::{KnowledgeBaseRepository, KnowledgeBaseRepositoryError};

pub struct SwitchKnowledgeBaseUseCase {
    repo: Arc<dyn KnowledgeBaseRepository>,
}

impl SwitchKnowledgeBaseUseCase {
    pub fn new(repo: Arc<dyn KnowledgeBaseRepository>) -> Self {
        Self { repo }
    }
    
    pub async fn execute(&self, id: &str) -> Result<crate::domain::entities::knowledge_base::KnowledgeBase, KnowledgeBaseRepositoryError> {
        let kb = self.repo.get_by_id(id).await?;
        self.repo.set_current(id).await?;
        Ok(kb)
    }
}

