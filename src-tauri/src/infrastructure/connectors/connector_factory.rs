use crate::domain::entities::resource_type::ResourceType;
use crate::domain::services::resource_connector::{ResourceConnector, ConnectorError};
use super::ssh_connector::SshConnector;

pub struct DefaultConnectorFactory;

impl crate::application::usecases::resource::execute_on_resources::ConnectorFactory for DefaultConnectorFactory {
    fn create(&self, resource_type: &ResourceType) -> Result<Box<dyn ResourceConnector>, ConnectorError> {
        match resource_type {
            ResourceType::SshServer => Ok(Box::new(SshConnector)),
            _ => Err(ConnectorError::UnsupportedOperation(format!("Resource type {:?} not yet implemented", resource_type))),
        }
    }
}

