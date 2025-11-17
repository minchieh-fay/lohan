use std::sync::Arc;
use crate::domain::services::llm_service::{LLMService, LLMError, ChatMessage, MessageRole};

pub struct ChatWithAIUseCase {
    llm_service: Arc<dyn LLMService>,
}

impl ChatWithAIUseCase {
    pub fn new(llm_service: Arc<dyn LLMService>) -> Self {
        Self { llm_service }
    }
    
    pub async fn execute(&self, user_message: String, context: Option<String>) -> Result<String, LLMError> {
        let mut messages = Vec::new();
        
        // 添加系统提示词
        let system_prompt = if let Some(ctx) = context {
            format!(
                "你是一个专业的AI运维助手。你正在帮助用户管理服务器和资源。\n\n当前上下文：\n{}\n\n请根据上下文信息，用专业、简洁的语言回答用户的问题。",
                ctx
            )
        } else {
            "你是一个专业的AI运维助手。你正在帮助用户管理服务器和资源。请用专业、简洁的语言回答用户的问题。".to_string()
        };
        
        messages.push(ChatMessage {
            role: MessageRole::System,
            content: system_prompt,
        });
        
        // 添加用户消息
        messages.push(ChatMessage {
            role: MessageRole::User,
            content: user_message,
        });
        
        // 调用LLM服务
        self.llm_service.chat(messages).await
    }
}

