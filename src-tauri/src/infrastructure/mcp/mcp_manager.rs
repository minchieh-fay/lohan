use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;
use crate::domain::services::mcp_tool::{MCPTool, MCPToolError, MCPToolInfo};

/// MCP工具管理器
pub struct MCPManager {
    tools: HashMap<String, Arc<dyn MCPTool>>,
}

impl MCPManager {
    pub fn new() -> Self {
        let mut manager = Self {
            tools: HashMap::new(),
        };
        
        // 注册所有工具
        manager.register_tools();
        
        manager
    }
    
    /// 注册所有工具
    fn register_tools(&mut self) {
        // 注册通用工具
        self.register(Arc::new(crate::infrastructure::mcp::tools::common::get_time::GetTimeTool));
        
        // 注册SSH工具
        self.register(Arc::new(crate::infrastructure::mcp::tools::ssh_tools::ssh_exec::SSHExecTool));
        
        // 未来可以在这里注册更多工具
        // self.register(Arc::new(PcapAnalyzerTool));
        // self.register(Arc::new(CoredumpAnalyzerTool));
    }
    
    /// 注册一个工具
    pub fn register(&mut self, tool: Arc<dyn MCPTool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }
    
    /// 列出所有可用工具
    pub fn list_tools(&self) -> Vec<MCPToolInfo> {
        self.tools
            .values()
            .map(|tool| {
                MCPToolInfo {
                    name: tool.name().to_string(),
                    description: tool.description().to_string(),
                    parameters: tool.parameters(),
                }
            })
            .collect()
    }
    
    /// 获取工具信息
    pub fn get_tool_info(&self, name: &str) -> Option<MCPToolInfo> {
        self.tools
            .get(name)
            .map(|tool| {
                MCPToolInfo {
                    name: tool.name().to_string(),
                    description: tool.description().to_string(),
                    parameters: tool.parameters(),
                }
            })
    }
    
    /// 执行工具
    pub async fn execute_tool(&self, name: &str, arguments: Value) -> Result<Value, MCPToolError> {
        let tool = self.tools
            .get(name)
            .ok_or_else(|| MCPToolError::ToolNotFound(name.to_string()))?;
        
        tool.execute(arguments).await
    }
    
    /// 检查工具是否存在
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }
}

impl Default for MCPManager {
    fn default() -> Self {
        Self::new()
    }
}

