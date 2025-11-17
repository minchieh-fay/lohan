use async_trait::async_trait;
use crate::domain::entities::knowledge_base::KnowledgeBase;

#[derive(Debug, thiserror::Error)]
pub enum KnowledgeBaseRepositoryError {
    #[error("Knowledge base not found: {0}")]
    NotFound(String),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Serialize error: {0}")]
    Serialize(String),
}

#[async_trait]
pub trait KnowledgeBaseRepository: Send + Sync {
    /// 列出所有知识库
    async fn list(&self) -> Result<Vec<KnowledgeBase>, KnowledgeBaseRepositoryError>;
    
    /// 根据ID获取知识库
    async fn get_by_id(&self, id: &str) -> Result<KnowledgeBase, KnowledgeBaseRepositoryError>;
    
    /// 创建新知识库
    async fn create(&self, kb: &KnowledgeBase) -> Result<(), KnowledgeBaseRepositoryError>;
    
    /// 删除知识库
    async fn delete(&self, id: &str) -> Result<(), KnowledgeBaseRepositoryError>;
    
    /// 更新知识库信息
    async fn update(&self, kb: &KnowledgeBase) -> Result<(), KnowledgeBaseRepositoryError>;
    
    /// 获取当前激活的知识库ID
    async fn get_current(&self) -> Result<Option<String>, KnowledgeBaseRepositoryError>;
    
    /// 设置当前激活的知识库
    async fn set_current(&self, id: &str) -> Result<(), KnowledgeBaseRepositoryError>;
}

