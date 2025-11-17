use async_trait::async_trait;
use serde_json::{json, Value};
use crate::domain::services::mcp_tool::{MCPTool, MCPToolError};
use crate::domain::entities::resource::ConnectionConfig;
use crate::infrastructure::connectors::ssh_connector::SshConnector;
use crate::domain::services::resource_connector::ResourceConnector;

/// SSH命令执行工具
pub struct SSHExecTool;

#[async_trait]
impl MCPTool for SSHExecTool {
    fn name(&self) -> &str {
        "ssh_execute_command"
    }
    
    fn description(&self) -> &str {
        "在远程SSH服务器上执行命令。需要提供服务器连接信息和要执行的命令。"
    }
    
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "host": {
                    "type": "string",
                    "description": "SSH服务器IP地址"
                },
                "port": {
                    "type": "number",
                    "description": "SSH端口，默认为22"
                },
                "username": {
                    "type": "string",
                    "description": "SSH用户名"
                },
                "password": {
                    "type": "string",
                    "description": "SSH密码"
                },
                "command": {
                    "type": "string",
                    "description": "要执行的命令"
                }
            },
            "required": ["host", "username", "password", "command"]
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<Value, MCPToolError> {
        // 解析参数
        let host = arguments
            .get("host")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MCPToolError::InvalidArguments("host is required".to_string()))?
            .to_string();
        
        let port = arguments
            .get("port")
            .and_then(|v| v.as_u64())
            .unwrap_or(22) as u16;
        
        let username = arguments
            .get("username")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MCPToolError::InvalidArguments("username is required".to_string()))?
            .to_string();
        
        let password = arguments
            .get("password")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MCPToolError::InvalidArguments("password is required".to_string()))?
            .to_string();
        
        let command = arguments
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MCPToolError::InvalidArguments("command is required".to_string()))?
            .to_string();
        
        // 创建SSH配置
        let config = ConnectionConfig::Ssh {
            host: host.clone(),
            port,
            username,
            password: Some(password),
            private_key: None,
            use_root: false,
            sudo_method: None,
            sudo_pass: None,
        };
        
        // 执行命令
        let connector = SshConnector;
        let output = connector.execute_command(&config, &command)
            .await
            .map_err(|e| MCPToolError::ExecutionFailed(e.to_string()))?;
        
        Ok(json!({
            "host": host,
            "command": command,
            "output": output,
            "success": true
        }))
    }
}

