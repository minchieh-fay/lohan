use async_trait::async_trait;
use crate::domain::entities::resource::{ConnectionConfig, ResourceStatus};
use crate::domain::entities::resource_type::ResourceType;

#[derive(Debug, thiserror::Error)]
pub enum ConnectorError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    #[error("Command execution failed: {0}")]
    CommandExecutionFailed(String),
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

/// 资源信息
#[derive(Debug, Clone)]
pub struct ResourceInfo {
    pub name: String,
    pub version: Option<String>,
    pub status: ResourceStatus,
    pub details: serde_json::Value,
}

/// 资源连接器接口 - 所有资源类型都需要实现此接口
#[async_trait]
pub trait ResourceConnector: Send + Sync {
    /// 测试连接
    async fn test_connection(&self, config: &ConnectionConfig) -> Result<(), ConnectorError>;
    
    /// 执行命令（如果资源支持）
    async fn execute_command(&self, config: &ConnectionConfig, command: &str) -> Result<String, ConnectorError>;
    
    /// 获取资源信息
    async fn get_info(&self, config: &ConnectionConfig) -> Result<ResourceInfo, ConnectorError>;
    
    /// 获取资源类型
    fn resource_type(&self) -> ResourceType;
}

