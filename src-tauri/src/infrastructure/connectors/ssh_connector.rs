use async_trait::async_trait;
use crate::domain::entities::resource::{ConnectionConfig, ResourceStatus};
use crate::domain::entities::resource_type::ResourceType;
use crate::domain::services::resource_connector::{ResourceConnector, ResourceInfo, ConnectorError};
use ssh2::Session;
use std::net::TcpStream;
use std::time::Duration;
use std::io::Read;

pub struct SshConnector;

#[async_trait]
impl ResourceConnector for SshConnector {
    async fn test_connection(&self, config: &ConnectionConfig) -> Result<(), ConnectorError> {
        match config {
            ConnectionConfig::Ssh { host, port, username, password, .. } => {
                // 使用tokio::task::spawn_blocking在异步上下文中执行同步SSH操作
                let host = host.clone();
                let port = *port;
                let username = username.clone();
                let password = password.clone();
                
                tokio::task::spawn_blocking(move || -> Result<(), ConnectorError> {
                    // 建立TCP连接
                    let addr = format!("{}:{}", host, port);
                    let tcp = TcpStream::connect_timeout(
                        &addr.parse().map_err(|e| ConnectorError::ConnectionFailed(format!("Invalid address: {}", e)))?,
                        Duration::from_secs(5)
                    ).map_err(|e| ConnectorError::ConnectionFailed(format!("Failed to connect to {}: {}", addr, e)))?;
                    
                    // 创建SSH会话
                    let mut sess = Session::new()
                        .map_err(|e| ConnectorError::ConnectionFailed(format!("Failed to create SSH session: {}", e)))?;
                    sess.set_tcp_stream(tcp);
                    sess.handshake().map_err(|e| ConnectorError::ConnectionFailed(format!("SSH handshake failed: {}", e)))?;
                    
                    // 尝试认证
                    if let Some(pwd) = password {
                        sess.userauth_password(&username, &pwd)
                            .map_err(|e| ConnectorError::AuthenticationFailed(format!("Password authentication failed: {}", e)))?;
                    } else {
                        return Err(ConnectorError::AuthenticationFailed("Password is required".to_string()));
                    }
                    
                    if !sess.authenticated() {
                        return Err(ConnectorError::AuthenticationFailed("Authentication failed".to_string()));
                    }
                    
                    // 测试执行一个简单命令
                    let mut channel = sess.channel_session()
                        .map_err(|e| ConnectorError::ConnectionFailed(format!("Failed to create channel: {}", e)))?;
                    channel.exec("echo 'Connection test successful'")
                        .map_err(|e| ConnectorError::ConnectionFailed(format!("Failed to execute command: {}", e)))?;
                    
                    // 读取输出（可选，只是为了确保命令执行成功）
                    let mut s = String::new();
                    channel.read_to_string(&mut s)
                        .map_err(|e| ConnectorError::ConnectionFailed(format!("Failed to read output: {}", e)))?;
                    channel.wait_close()
                        .map_err(|e| ConnectorError::ConnectionFailed(format!("Failed to close channel: {}", e)))?;
                    
                    Ok(())
                }).await
                .map_err(|e| ConnectorError::ConnectionFailed(format!("Task error: {}", e)))?
            }
            _ => Err(ConnectorError::UnsupportedOperation("Not SSH config".to_string())),
        }
    }
    
    async fn execute_command(&self, config: &ConnectionConfig, command: &str) -> Result<String, ConnectorError> {
        match config {
            ConnectionConfig::Ssh { host, .. } => {
                // TODO: Implement SSH command execution
                println!("Executing command on {}: {}", host, command);
                Ok(format!("Command executed: {}", command))
            }
            _ => Err(ConnectorError::UnsupportedOperation("Not SSH config".to_string())),
        }
    }
    
    async fn get_info(&self, config: &ConnectionConfig) -> Result<ResourceInfo, ConnectorError> {
        match config {
            ConnectionConfig::Ssh { host, .. } => {
                // TODO: Implement SSH info retrieval
                Ok(ResourceInfo {
                    name: host.clone(),
                    version: None,
                    status: ResourceStatus::Online,
                    details: serde_json::json!({}),
                })
            }
            _ => Err(ConnectorError::UnsupportedOperation("Not SSH config".to_string())),
        }
    }
    
    fn resource_type(&self) -> ResourceType {
        ResourceType::SshServer
    }
}

