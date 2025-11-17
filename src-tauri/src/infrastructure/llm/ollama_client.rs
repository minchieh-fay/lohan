use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::domain::services::llm_service::{LLMService, LLMError, ChatMessage, MessageRole};

/// Ollama API客户端
pub struct OllamaClient {
    base_url: String,
    model: String,
    client: reqwest::Client,
}

impl OllamaClient {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            base_url,
            model,
            client: reqwest::Client::new(),
        }
    }
    
    /// 从配置创建（默认使用局域网Ollama）
    pub fn from_config() -> Self {
        Self::new(
            "http://10.35.148.111:11434".to_string(),
            "qwen3:8b".to_string(), // 默认使用qwen3:8b模型
        )
    }
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

#[derive(Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    created_at: Option<String>,
    message: OllamaMessageResponse,
    done: bool,
    #[serde(default)]
    done_reason: Option<String>,
}

#[derive(Deserialize)]
struct OllamaMessageResponse {
    role: String,
    content: String,
    #[serde(default)]
    thinking: Option<String>, // qwen3模型特有的thinking字段
}

#[derive(Deserialize)]
struct ModelsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Deserialize)]
struct ModelInfo {
    name: String,
}

#[async_trait]
impl LLMService for OllamaClient {
    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, LLMError> {
        // 转换为Ollama格式
        let ollama_messages: Vec<OllamaMessage> = messages
            .into_iter()
            .map(|msg| OllamaMessage {
                role: match msg.role {
                    MessageRole::System => "system".to_string(),
                    MessageRole::User => "user".to_string(),
                    MessageRole::Assistant => "assistant".to_string(),
                },
                content: msg.content,
            })
            .collect();
        
        let request = ChatRequest {
            model: self.model.clone(),
            messages: ollama_messages,
            stream: false,
        };
        
        let url = format!("{}/api/chat", self.base_url);
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| LLMError::ConnectionFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(LLMError::RequestFailed(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }
        
        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| LLMError::InvalidResponse(e.to_string()))?;
        
        Ok(chat_response.message.content)
    }
    
    async fn health_check(&self) -> Result<(), LLMError> {
        let url = format!("{}/api/tags", self.base_url);
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| LLMError::ConnectionFailed(e.to_string()))?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            Err(LLMError::ConnectionFailed(format!(
                "Health check failed: HTTP {}",
                response.status()
            )))
        }
    }
    
    async fn list_models(&self) -> Result<Vec<String>, LLMError> {
        let url = format!("{}/api/tags", self.base_url);
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| LLMError::ConnectionFailed(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(LLMError::RequestFailed(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }
        
        let models_response: ModelsResponse = response
            .json()
            .await
            .map_err(|e| LLMError::InvalidResponse(e.to_string()))?;
        
        Ok(models_response.models.into_iter().map(|m| m.name).collect())
    }
}

