// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod domain;
mod application;
mod infrastructure;

use std::sync::Arc;
use tauri::Manager;
use infrastructure::llm::ollama_client::OllamaClient;
use domain::services::llm_service::LLMService;
use application::usecases::llm::chat_with_ai::ChatWithAIUseCase;
use domain::entities::resource::ConnectionConfig;
use infrastructure::connectors::ssh_connector::SshConnector;
use domain::services::resource_connector::ResourceConnector;
use infrastructure::mcp::mcp_manager::MCPManager;
use serde_json::Value;

// Application state
struct AppState {
    llm_service: Arc<dyn LLMService>,
    chat_use_case: Arc<ChatWithAIUseCase>,
    mcp_manager: Arc<MCPManager>,
}

#[tauri::command]
async fn chat_with_ai(message: String, context: Option<String>) -> Result<String, String> {
    // TODO: Get from app state
    let llm_service: Arc<dyn LLMService> = Arc::new(OllamaClient::from_config());
    let use_case = ChatWithAIUseCase::new(llm_service);
    
    use_case.execute(message, context)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn check_llm_health() -> Result<bool, String> {
    let llm_service: Arc<dyn LLMService> = Arc::new(OllamaClient::from_config());
    llm_service.health_check()
        .await
        .map(|_| true)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_llm_models() -> Result<Vec<String>, String> {
    let llm_service: Arc<dyn LLMService> = Arc::new(OllamaClient::from_config());
    llm_service.list_models()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn test_ssh_connection(
    host: String,
    port: u16,
    username: String,
    password: String,
) -> Result<String, String> {
    let config = ConnectionConfig::Ssh {
        host,
        port,
        username,
        password: Some(password),
        private_key: None,
        use_root: false,
        sudo_method: None,
        sudo_pass: None,
    };
    
    let connector = SshConnector;
    connector.test_connection(&config)
        .await
        .map(|_| "连接测试成功！".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_mcp_tools() -> Result<Vec<serde_json::Value>, String> {
    let manager = MCPManager::new();
    let tools = manager.list_tools();
    Ok(tools.into_iter().map(|t| serde_json::json!({
        "name": t.name,
        "description": t.description,
        "parameters": t.parameters
    })).collect())
}

#[tauri::command]
async fn execute_mcp_tool(tool_name: String, arguments: Value) -> Result<Value, String> {
    let manager = MCPManager::new();
    manager.execute_tool(&tool_name, arguments)
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize LLM service
            let llm_service: Arc<dyn LLMService> = Arc::new(OllamaClient::from_config());
            let chat_use_case = Arc::new(ChatWithAIUseCase::new(llm_service.clone()));
            let mcp_manager = Arc::new(MCPManager::new());
            
            app.manage(AppState {
                llm_service,
                chat_use_case,
                mcp_manager,
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            chat_with_ai,
            check_llm_health,
            list_llm_models,
            test_ssh_connection,
            list_mcp_tools,
            execute_mcp_tool,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

