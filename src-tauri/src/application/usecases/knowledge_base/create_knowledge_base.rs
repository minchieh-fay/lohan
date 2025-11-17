use std::sync::Arc;
use std::path::PathBuf;
use crate::domain::repositories::knowledge_base_repository::{KnowledgeBaseRepository, KnowledgeBaseRepositoryError};
use crate::domain::entities::knowledge_base::KnowledgeBase;

pub struct CreateKnowledgeBaseUseCase {
    repo: Arc<dyn KnowledgeBaseRepository>,
}

impl CreateKnowledgeBaseUseCase {
    pub fn new(repo: Arc<dyn KnowledgeBaseRepository>) -> Self {
        Self { repo }
    }
    
    pub async fn execute(&self, name: String, description: Option<String>) -> Result<KnowledgeBase, KnowledgeBaseRepositoryError> {
        let id = Self::generate_id(&name);
        let base_path = PathBuf::from("./doc");
        let mut kb = KnowledgeBase::new(id.clone(), name, &base_path);
        kb.description = description;
        
        self.repo.create(&kb).await?;
        Self::initialize_structure(&kb.path)?;
        
        Ok(kb)
    }
    
    fn generate_id(name: &str) -> String {
        name.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .trim_matches('-')
            .to_string()
    }
    
    fn initialize_structure(path: &PathBuf) -> Result<(), KnowledgeBaseRepositoryError> {
        let dirs = vec![
            "company",
            "environments/production",
            "environments/staging",
            "environments/development",
            "services",
            "operations/deployment",
            "operations/monitoring",
            "operations/emergency",
            "standards",
            "history/incidents",
            "history/solutions",
        ];
        
        for dir in dirs {
            std::fs::create_dir_all(path.join(dir))
                .map_err(|e| KnowledgeBaseRepositoryError::IO(e))?;
        }
        
        Ok(())
    }
}

