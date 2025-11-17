use async_trait::async_trait;
use crate::domain::entities::resource::Resource;
use crate::domain::entities::resource_type::ResourceType;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Serialize error: {0}")]
    Serialize(String),
}

#[async_trait]
pub trait ResourceRepository: Send + Sync {
    /// 列出所有资源
    async fn list(&self) -> Result<Vec<Resource>, RepositoryError>;
    
    /// 根据ID获取资源
    async fn get_by_id(&self, id: &str) -> Result<Resource, RepositoryError>;
    
    /// 创建新资源
    async fn create(&self, resource: &Resource) -> Result<(), RepositoryError>;
    
    /// 删除资源
    async fn delete(&self, id: &str) -> Result<(), RepositoryError>;
    
    /// 更新资源信息
    async fn update(&self, resource: &Resource) -> Result<(), RepositoryError>;
    
    /// 根据类型筛选资源
    async fn list_by_type(&self, resource_type: &ResourceType) -> Result<Vec<Resource>, RepositoryError>;
}

