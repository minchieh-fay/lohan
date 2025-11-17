use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum MCPToolError {
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

/// MCP工具接口 - 所有MCP工具都需要实现此接口
#[async_trait]
pub trait MCPTool: Send + Sync {
    /// 工具名称（唯一标识符）
    fn name(&self) -> &str;
    
    /// 工具描述（用于AI理解工具用途）
    fn description(&self) -> &str;
    
    /// 工具参数定义（JSON Schema格式）
    fn parameters(&self) -> Value;
    
    /// 执行工具
    /// 
    /// # Arguments
    /// * `arguments` - JSON格式的参数
    /// 
    /// # Returns
    /// * `Ok(Value)` - 执行结果（JSON格式）
    /// * `Err(MCPToolError)` - 执行错误
    async fn execute(&self, arguments: Value) -> Result<Value, MCPToolError>;
}

/// MCP工具信息（用于向AI描述可用工具）
#[derive(Debug, Clone)]
pub struct MCPToolInfo {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

impl MCPToolInfo {
    pub fn from_tool<T: MCPTool>(tool: &T) -> Self {
        Self {
            name: tool.name().to_string(),
            description: tool.description().to_string(),
            parameters: tool.parameters(),
        }
    }
}

