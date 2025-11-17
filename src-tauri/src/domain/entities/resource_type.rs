use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    /// SSH服务器
    SshServer,
    /// Rancher平台
    Rancher,
    /// Kubernetes集群
    Kubernetes,
    /// Docker Swarm集群
    DockerSwarm,
    /// AWS云服务
    Aws,
    /// Azure云服务
    Azure,
    /// GCP云服务
    Gcp,
    /// 自定义类型
    Custom(String),
}

impl ResourceType {
    pub fn display_name(&self) -> &str {
        match self {
            ResourceType::SshServer => "SSH服务器",
            ResourceType::Rancher => "Rancher平台",
            ResourceType::Kubernetes => "Kubernetes集群",
            ResourceType::DockerSwarm => "Docker Swarm",
            ResourceType::Aws => "AWS云服务",
            ResourceType::Azure => "Azure云服务",
            ResourceType::Gcp => "GCP云服务",
            ResourceType::Custom(name) => name,
        }
    }
    
    pub fn icon(&self) -> &str {
        match self {
            ResourceType::SshServer => "🖥️",
            ResourceType::Rancher => "🐄",
            ResourceType::Kubernetes => "☸️",
            ResourceType::DockerSwarm => "🐳",
            ResourceType::Aws => "☁️",
            ResourceType::Azure => "☁️",
            ResourceType::Gcp => "☁️",
            ResourceType::Custom(_) => "📦",
        }
    }
}

