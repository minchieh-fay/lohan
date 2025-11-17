use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum LLMError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("Model not available: {0}")]
    ModelNotAvailable(String),
}

/// LLM服务接口
#[async_trait]
pub trait LLMService: Send + Sync {
    /// 发送消息并获取回复
    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, LLMError>;
    
    /// 检查服务是否可用
    async fn health_check(&self) -> Result<(), LLMError>;
    
    /// 列出可用的模型
    async fn list_models(&self) -> Result<Vec<String>, LLMError>;
}

/// 聊天消息
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

