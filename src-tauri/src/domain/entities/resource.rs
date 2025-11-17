use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::resource_type::ResourceType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    pub name: String,
    pub resource_type: ResourceType,
    pub connection_config: ConnectionConfig,
    pub metadata: ResourceMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConnectionConfig {
    /// SSH连接配置
    Ssh {
        host: String,
        port: u16,
        username: String,
        password: Option<String>,
        private_key: Option<String>,
        use_root: bool,
        sudo_method: Option<String>,
        sudo_pass: Option<String>,
    },
    /// Rancher连接配置
    Rancher {
        url: String,
        username: String,
        password: String,
        api_token: Option<String>,
    },
    /// Kubernetes连接配置
    Kubernetes {
        kubeconfig_path: String,
        context: Option<String>,
        namespace: Option<String>,
    },
    /// Docker Swarm连接配置
    DockerSwarm {
        host: String,
        port: Option<u16>,
        tls: bool,
        cert_path: Option<String>,
    },
    /// AWS连接配置
    Aws {
        region: String,
        access_key_id: String,
        secret_access_key: String,
        profile: Option<String>,
    },
    /// Azure连接配置
    Azure {
        subscription_id: String,
        tenant_id: String,
        client_id: String,
        client_secret: String,
    },
    /// GCP连接配置
    Gcp {
        project_id: String,
        service_account_key: String,
        region: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceMetadata {
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub environment: Option<String>, // production, staging, development
    pub group: Option<String>,
    pub status: ResourceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ResourceStatus {
    #[default]
    Unknown,
    Online,
    Offline,
    Error(String),
}

