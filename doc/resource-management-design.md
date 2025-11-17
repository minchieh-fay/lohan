# 资源管理扩展设计

## 📋 需求概述

运维对象不仅限于传统SSH服务器，还应支持：
- **SSH服务器**：传统的Linux/Unix服务器
- **Rancher平台**：Rancher地址 + 账号密码
- **Kubernetes集群**：kubeconfig文件
- **Docker Swarm**：Docker Swarm集群
- **云平台**：AWS/Azure/GCP等云服务
- **其他**：可扩展的资源类型

## 🎯 设计原则

### 1. 统一抽象
将"服务器列表"抽象为"资源列表"，所有运维目标都是"资源"。

### 2. 类型化资源
不同类型的资源有不同的连接方式和操作能力，但都遵循统一的接口。

### 3. 可扩展性
通过插件化设计，支持未来添加新的资源类型。

## 🏗️ Clean Architecture 设计

### 1. 领域层（Domain Layer）

#### 资源类型枚举

```rust
// src-tauri/src/domain/entities/resource_type.rs

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
```

#### 资源实体

```rust
// src-tauri/src/domain/entities/resource.rs

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResourceStatus {
    Online,
    Offline,
    Unknown,
    Error(String),
}
```

#### 资源连接器接口

```rust
// src-tauri/src/domain/services/resource_connector.rs

use async_trait::async_trait;

/// 资源连接器接口 - 所有资源类型都需要实现此接口
#[async_trait]
pub trait ResourceConnector: Send + Sync {
    /// 测试连接
    async fn test_connection(&self, config: &ConnectionConfig) -> Result<(), ConnectorError>;
    
    /// 执行命令（如果资源支持）
    async fn execute_command(&self, config: &ConnectionConfig, command: &str) -> Result<String, ConnectorError>;
    
    /// 获取资源信息
    async fn get_info(&self, config: &ConnectionConfig) -> Result<ResourceInfo, ConnectorError>;
    
    /// 获取资源类型
    fn resource_type(&self) -> ResourceType;
}

/// 资源信息
#[derive(Debug, Clone)]
pub struct ResourceInfo {
    pub name: String,
    pub version: Option<String>,
    pub status: ResourceStatus,
    pub details: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectorError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    #[error("Command execution failed: {0}")]
    CommandExecutionFailed(String),
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}
```

### 2. 应用层（Application Layer）

#### 资源管理用例

```rust
// src-tauri/src/application/usecases/resource/list_resources.rs

pub struct ListResourcesUseCase {
    repo: Arc<dyn ResourceRepository>,
}

impl ListResourcesUseCase {
    pub fn execute(&self, filter: Option<ResourceFilter>) -> Result<Vec<Resource>, UseCaseError> {
        self.repo.list(filter)
            .map_err(|e| UseCaseError::Repository(e))
    }
}

#[derive(Debug, Clone)]
pub struct ResourceFilter {
    pub resource_type: Option<ResourceType>,
    pub environment: Option<String>,
    pub group: Option<String>,
    pub status: Option<ResourceStatus>,
}
```

```rust
// src-tauri/src/application/usecases/resource/execute_on_resources.rs

pub struct ExecuteOnResourcesUseCase {
    connector_factory: Arc<dyn ResourceConnectorFactory>,
}

impl ExecuteOnResourcesUseCase {
    pub async fn execute(
        &self,
        resource_ids: Vec<String>,
        command: &str,
    ) -> Result<HashMap<String, ExecutionResult>, UseCaseError> {
        let mut results = HashMap::new();
        
        for resource_id in resource_ids {
            // 获取资源
            let resource = self.repo.get_by_id(&resource_id)?;
            
            // 获取对应的连接器
            let connector = self.connector_factory.create(&resource.resource_type)?;
            
            // 执行命令
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

#[derive(Debug, Clone)]
pub enum ExecutionResult {
    Success(String),
    Error(String),
    Unsupported,
}
```

### 3. 基础设施层（Infrastructure Layer）

#### SSH连接器实现

```rust
// src-tauri/src/infrastructure/connectors/ssh_connector.rs

pub struct SshConnector;

#[async_trait]
impl ResourceConnector for SshConnector {
    async fn test_connection(&self, config: &ConnectionConfig) -> Result<(), ConnectorError> {
        if let ConnectionConfig::Ssh { host, port, username, password, .. } = config {
            // SSH连接测试逻辑
            // ...
            Ok(())
        } else {
            Err(ConnectorError::UnsupportedOperation("Not SSH config".to_string()))
        }
    }
    
    async fn execute_command(&self, config: &ConnectionConfig, command: &str) -> Result<String, ConnectorError> {
        // SSH命令执行逻辑
        // ...
    }
    
    fn resource_type(&self) -> ResourceType {
        ResourceType::SshServer
    }
    
    // ...
}
```

#### Rancher连接器实现

```rust
// src-tauri/src/infrastructure/connectors/rancher_connector.rs

pub struct RancherConnector;

#[async_trait]
impl ResourceConnector for RancherConnector {
    async fn test_connection(&self, config: &ConnectionConfig) -> Result<(), ConnectorError> {
        if let ConnectionConfig::Rancher { url, username, password, .. } = config {
            // Rancher API连接测试
            let client = reqwest::Client::new();
            let response = client
                .post(&format!("{}/v3-public/localProviders/local?action=login", url))
                .json(&json!({
                    "username": username,
                    "password": password,
                }))
                .send()
                .await
                .map_err(|e| ConnectorError::ConnectionFailed(e.to_string()))?;
            
            if response.status().is_success() {
                Ok(())
            } else {
                Err(ConnectorError::AuthenticationFailed("Invalid credentials".to_string()))
            }
        } else {
            Err(ConnectorError::UnsupportedOperation("Not Rancher config".to_string()))
        }
    }
    
    async fn execute_command(&self, config: &ConnectionConfig, command: &str) -> Result<String, ConnectorError> {
        // Rancher API调用逻辑
        // 例如：创建/删除工作负载、执行kubectl命令等
        // ...
    }
    
    fn resource_type(&self) -> ResourceType {
        ResourceType::Rancher
    }
    
    // ...
}
```

#### Kubernetes连接器实现

```rust
// src-tauri/src/infrastructure/connectors/kubernetes_connector.rs

pub struct KubernetesConnector;

#[async_trait]
impl ResourceConnector for KubernetesConnector {
    async fn test_connection(&self, config: &ConnectionConfig) -> Result<(), ConnectorError> {
        if let ConnectionConfig::Kubernetes { kubeconfig_path, context, .. } = config {
            // 加载kubeconfig并测试连接
            let config = kube::Config::from_kubeconfig(kubeconfig_path)
                .await
                .map_err(|e| ConnectorError::ConnectionFailed(e.to_string()))?;
            
            // 测试API连接
            let client = kube::Client::try_from(config)
                .map_err(|e| ConnectorError::ConnectionFailed(e.to_string()))?;
            
            // 尝试列出namespaces
            let _namespaces: kube::api::List<kube::api::Namespace> = kube::Api::all(client)
                .list(&Default::default())
                .await
                .map_err(|e| ConnectorError::ConnectionFailed(e.to_string()))?;
            
            Ok(())
        } else {
            Err(ConnectorError::UnsupportedOperation("Not Kubernetes config".to_string()))
        }
    }
    
    async fn execute_command(&self, config: &ConnectionConfig, command: &str) -> Result<String, ConnectorError> {
        // 解析kubectl命令并执行
        // 例如：kubectl get pods, kubectl apply -f xxx.yaml等
        // ...
    }
    
    fn resource_type(&self) -> ResourceType {
        ResourceType::Kubernetes
    }
    
    // ...
}
```

#### 连接器工厂

```rust
// src-tauri/src/infrastructure/connectors/connector_factory.rs

pub trait ResourceConnectorFactory: Send + Sync {
    fn create(&self, resource_type: &ResourceType) -> Result<Box<dyn ResourceConnector>, FactoryError>;
}

pub struct DefaultConnectorFactory;

impl ResourceConnectorFactory for DefaultConnectorFactory {
    fn create(&self, resource_type: &ResourceType) -> Result<Box<dyn ResourceConnector>, FactoryError> {
        match resource_type {
            ResourceType::SshServer => Ok(Box::new(SshConnector)),
            ResourceType::Rancher => Ok(Box::new(RancherConnector)),
            ResourceType::Kubernetes => Ok(Box::new(KubernetesConnector)),
            ResourceType::DockerSwarm => Ok(Box::new(DockerSwarmConnector)),
            ResourceType::Aws => Ok(Box::new(AwsConnector)),
            ResourceType::Azure => Ok(Box::new(AzureConnector)),
            ResourceType::Gcp => Ok(Box::new(GcpConnector)),
            ResourceType::Custom(_) => Err(FactoryError::UnsupportedType(resource_type.clone())),
        }
    }
}
```

### 4. 表示层（Presentation Layer）

#### UI组件设计

```vue
<!-- 资源列表组件 -->
<template>
  <div class="resource-list">
    <div class="resource-list-header">
      <h2>资源列表</h2>
      <button @click="showAddResourceDialog">+ 添加</button>
    </div>
    
    <!-- 资源类型筛选 -->
    <div class="resource-filter">
      <select v-model="selectedType" @change="filterResources">
        <option value="">全部类型</option>
        <option value="ssh">🖥️ SSH服务器</option>
        <option value="rancher">🐄 Rancher</option>
        <option value="kubernetes">☸️ Kubernetes</option>
        <option value="docker-swarm">🐳 Docker Swarm</option>
        <option value="aws">☁️ AWS</option>
      </select>
    </div>
    
    <!-- 资源列表 -->
    <div class="resources">
      <div
        v-for="resource in filteredResources"
        :key="resource.id"
        class="resource-item"
        :class="{ selected: selectedResourceIds.includes(resource.id) }"
        @click="toggleResourceSelection(resource.id)"
      >
        <div class="resource-icon">{{ getResourceIcon(resource.resource_type) }}</div>
        <div class="resource-info">
          <div class="resource-name">{{ resource.name }}</div>
          <div class="resource-details">{{ getResourceDetails(resource) }}</div>
        </div>
        <div class="resource-status" :class="resource.metadata.status">
          {{ getStatusText(resource.metadata.status) }}
        </div>
        <div class="resource-actions">
          <button @click.stop="editResource(resource)">✎</button>
          <button @click.stop="deleteResource(resource.id)">×</button>
        </div>
      </div>
    </div>
  </div>
</template>
```

#### 添加资源对话框

```vue
<!-- 添加资源对话框 -->
<template>
  <div class="modal-overlay" @click="closeDialog">
    <div class="modal-content" @click.stop>
      <h2>添加资源</h2>
      
      <!-- 资源类型选择 -->
      <div class="form-group">
        <label>资源类型</label>
        <select v-model="resourceForm.resource_type" @change="onResourceTypeChange">
          <option value="ssh">🖥️ SSH服务器</option>
          <option value="rancher">🐄 Rancher平台</option>
          <option value="kubernetes">☸️ Kubernetes集群</option>
          <option value="docker-swarm">🐳 Docker Swarm</option>
          <option value="aws">☁️ AWS云服务</option>
        </select>
      </div>
      
      <!-- SSH配置 -->
      <div v-if="resourceForm.resource_type === 'ssh'" class="resource-config">
        <div class="form-group">
          <label>IP地址</label>
          <input v-model="resourceForm.ssh.host" type="text" />
        </div>
        <div class="form-group">
          <label>SSH端口</label>
          <input v-model.number="resourceForm.ssh.port" type="number" />
        </div>
        <!-- ... 其他SSH配置 -->
      </div>
      
      <!-- Rancher配置 -->
      <div v-if="resourceForm.resource_type === 'rancher'" class="resource-config">
        <div class="form-group">
          <label>Rancher地址</label>
          <input v-model="resourceForm.rancher.url" type="text" placeholder="https://rancher.example.com" />
        </div>
        <div class="form-group">
          <label>用户名</label>
          <input v-model="resourceForm.rancher.username" type="text" />
        </div>
        <div class="form-group">
          <label>密码</label>
          <input v-model="resourceForm.rancher.password" type="password" />
        </div>
      </div>
      
      <!-- Kubernetes配置 -->
      <div v-if="resourceForm.resource_type === 'kubernetes'" class="resource-config">
        <div class="form-group">
          <label>Kubeconfig文件路径</label>
          <input v-model="resourceForm.kubernetes.kubeconfig_path" type="text" />
          <button @click="selectKubeconfigFile">选择文件</button>
        </div>
        <div class="form-group">
          <label>Context（可选）</label>
          <input v-model="resourceForm.kubernetes.context" type="text" />
        </div>
        <div class="form-group">
          <label>Namespace（可选）</label>
          <input v-model="resourceForm.kubernetes.namespace" type="text" />
        </div>
      </div>
      
      <!-- ... 其他资源类型的配置 -->
      
      <div class="modal-actions">
        <button @click="closeDialog">取消</button>
        <button @click="testConnection">测试连接</button>
        <button @click="saveResource">保存</button>
      </div>
    </div>
  </div>
</template>
```

## 📁 目录结构

```
src-tauri/src/
├── domain/
│   ├── entities/
│   │   ├── resource.rs              # 资源实体
│   │   ├── resource_type.rs         # 资源类型枚举
│   │   └── connection_config.rs     # 连接配置
│   ├── repositories/
│   │   └── resource_repository.rs   # 资源仓储接口
│   └── services/
│       └── resource_connector.rs    # 资源连接器接口
├── application/
│   └── usecases/
│       └── resource/
│           ├── list_resources.rs
│           ├── add_resource.rs
│           ├── execute_on_resources.rs
│           └── test_connection.rs
└── infrastructure/
    ├── repositories/
    │   └── file_system_resource_repository.rs
    └── connectors/
        ├── ssh_connector.rs
        ├── rancher_connector.rs
        ├── kubernetes_connector.rs
        ├── docker_swarm_connector.rs
        ├── aws_connector.rs
        ├── azure_connector.rs
        ├── gcp_connector.rs
        └── connector_factory.rs
```

## 🔄 使用场景示例

### 场景1：混合资源管理
```
用户选择：
- Web-01 (SSH服务器)
- Rancher-Prod (Rancher平台)
- K8s-Cluster (Kubernetes集群)

用户指令："帮我重启所有Web服务"

AI处理：
1. 识别Web服务部署在哪些资源上
2. SSH服务器：执行 systemctl restart nginx
3. Rancher：调用API重启工作负载
4. Kubernetes：执行 kubectl rollout restart deployment/web-app
```

### 场景2：Kubernetes资源
```
用户添加Kubernetes资源：
- kubeconfig: ~/.kube/config-prod.yaml
- context: production
- namespace: default

用户指令："查看所有Pod状态"

AI执行：
kubectl get pods --context=production -n default
```

### 场景3：Rancher资源
```
用户添加Rancher资源：
- URL: https://rancher.company.com
- 用户名/密码

用户指令："部署新版本到生产环境"

AI执行：
1. 通过Rancher API创建/更新工作负载
2. 或调用kubectl命令（如果Rancher支持）
```

## 🎯 实现优先级

### Phase 1 - MVP
- [ ] 资源实体和类型定义
- [ ] SSH连接器（已有基础）
- [ ] 资源列表UI（支持类型筛选）
- [ ] 添加资源对话框（支持SSH和Kubernetes）

### Phase 2 - 增强
- [ ] Rancher连接器
- [ ] Docker Swarm连接器
- [ ] 云平台连接器（AWS/Azure/GCP）
- [ ] 资源分组和标签

### Phase 3 - 高级
- [ ] 插件化资源类型
- [ ] 资源模板
- [ ] 批量导入资源
- [ ] 资源监控和告警

## 💡 设计优势

1. **统一抽象**：所有运维目标都是"资源"，UI和逻辑统一
2. **类型安全**：Rust的类型系统确保连接配置与资源类型匹配
3. **可扩展**：新增资源类型只需实现`ResourceConnector`接口
4. **向后兼容**：现有的SSH服务器可以无缝迁移为资源类型之一

