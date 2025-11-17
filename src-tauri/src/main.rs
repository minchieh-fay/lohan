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

// Application state
struct AppState {
    llm_service: Arc<dyn LLMService>,
    chat_use_case: Arc<ChatWithAIUseCase>,
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

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize LLM service
            let llm_service: Arc<dyn LLMService> = Arc::new(OllamaClient::from_config());
            let chat_use_case = Arc::new(ChatWithAIUseCase::new(llm_service.clone()));
            
            app.manage(AppState {
                llm_service,
                chat_use_case,
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            chat_with_ai,
            check_llm_health,
            list_llm_models,
            test_ssh_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

