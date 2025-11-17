use std::sync::Arc;
use std::collections::HashMap;
use crate::domain::repositories::resource_repository::{ResourceRepository, RepositoryError};
use crate::domain::services::resource_connector::{ResourceConnector, ConnectorError};

#[derive(Debug, Clone)]
pub enum ExecutionResult {
    Success(String),
    Error(String),
    Unsupported,
}

pub struct ExecuteOnResourcesUseCase {
    repo: Arc<dyn ResourceRepository>,
    connector_factory: Arc<dyn ConnectorFactory>,
}

pub trait ConnectorFactory: Send + Sync {
    fn create(&self, resource_type: &crate::domain::entities::resource_type::ResourceType) -> Result<Box<dyn ResourceConnector>, ConnectorError>;
}

impl ExecuteOnResourcesUseCase {
    pub fn new(repo: Arc<dyn ResourceRepository>, connector_factory: Arc<dyn ConnectorFactory>) -> Self {
        Self { repo, connector_factory }
    }
    
    pub async fn execute(
        &self,
        resource_ids: Vec<String>,
        command: &str,
    ) -> Result<HashMap<String, ExecutionResult>, RepositoryError> {
        let mut results = HashMap::new();
        
        for resource_id in resource_ids {
            let resource = self.repo.get_by_id(&resource_id).await?;
            let connector = self.connector_factory.create(&resource.resource_type)
                .map_err(|e| RepositoryError::Parse(e.to_string()))?;
            
            match connector.execute_command(&resource.connection_config, command).await {
                Ok(output) => {
                    results.insert(resource_id, ExecutionResult::Success(output));
                }
                Err(e) => {
                    results.insert(resource_id, ExecutionResult::Error(e.to_string()));
                }
            }
        }
        
        Ok(results)
    }
}

