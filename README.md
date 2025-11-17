# Lohan

Lohan is an AI-powered system analyst and operations assistant. It automates system diagnostics, core dump analysis, malware cleanup, and data repair, helping you maintain robust and healthy systems.

## 项目概述

Lohan是一个AI驱动的运维工具和Bug修复助手，专为运维、技术支持和研发人员设计。

## 测试的llm
ip: 10.35.148.111
端口: 11434
ollama

## 核心功能

- **多知识库管理**：支持多个产品/平台的知识库切换，每个知识库独立管理
- **统一资源管理**：不仅支持SSH服务器，还支持Rancher、Kubernetes、Docker Swarm、云平台等多种资源类型
- **AI对话**：右侧聊天框与AI对话，让AI帮助解决问题
- **知识库驱动**：AI基于当前知识库的配置和文档来执行任务
- **自动化运维**：AI可以执行各种运维任务，如：
  - "这些机器帮忙装下docker"
  - "帮忙查一下病毒"
  - "这些机器的opt目录有pcap抓包，帮忙分析为什么会断线"
  - "帮忙分析coredump"
  - "重启Rancher上的所有Web服务"
  - "查看K8s集群的Pod状态"
  - 等等...

## 知识库管理

### 多知识库支持

Lohan支持管理多个产品/平台的知识库，每个知识库独立存储和管理：

```
./doc/
├── product-a/          # 产品A的知识库
│   ├── company/
│   ├── environments/
│   ├── services/
│   └── operations/
├── product-b/          # 产品B的知识库
│   ├── company/
│   ├── environments/
│   ├── services/
│   └── operations/
└── platform-x/        # 平台X的知识库
    ├── company/
    ├── environments/
    ├── services/
    └── operations/
```

### 知识库切换

- **切换界面**：顶部工具栏显示当前知识库，支持快速切换
- **独立配置**：每个知识库包含独立的服务器配置、服务定义、运维手册等
- **上下文隔离**：切换知识库后，AI对话基于新知识库的上下文

### 使用场景

**场景1：多产品运维**
- 运维工程师需要同时管理产品A和产品B
- 产品A使用MySQL，产品B使用PostgreSQL
- 通过切换知识库，AI自动使用对应产品的数据库配置

**场景2：多平台管理**
- 同一产品部署在多个平台（生产、测试、开发）
- 每个平台有不同的服务器、端口、路径配置
- 切换知识库后，AI使用对应平台的配置执行任务

## 资源管理

### 支持的资源类型

Lohan支持多种运维目标，统一抽象为"资源"：

- **🖥️ SSH服务器**：传统的Linux/Unix服务器，通过SSH连接
- **🐄 Rancher平台**：Rancher地址 + 账号密码，通过Rancher API操作
- **☸️ Kubernetes集群**：kubeconfig文件，支持kubectl命令
- **🐳 Docker Swarm**：Docker Swarm集群管理
- **☁️ 云平台**：AWS、Azure、GCP等云服务
- **📦 自定义类型**：可扩展支持其他资源类型

### 统一管理

所有资源类型在左侧列表中统一显示，支持：
- 多选资源进行操作
- 按类型筛选
- 按环境/分组管理
- 统一的状态监控

### 使用场景

**场景1：混合资源**
```
选择资源：
- Web-01 (SSH服务器)
- Rancher-Prod (Rancher平台)
- K8s-Cluster (Kubernetes集群)

用户指令："重启所有Web服务"
→ AI自动识别资源类型，使用对应方式执行
```

**场景2：Kubernetes集群**
```
添加资源：
- 类型：Kubernetes
- kubeconfig: ~/.kube/config-prod.yaml
- context: production

用户指令："查看所有Pod状态"
→ AI执行 kubectl get pods --context=production
```

详细设计文档请参考：[资源管理设计](./doc/resource-management-design.md)

## 技术架构

- **开发语言**: Rust + JavaScript
- **框架**: Tauri
- **架构模式**: Clean Architecture（清洁架构）
- **AI能力**: 集成本地LLM模型（Qwen2-Coder系列）
- **工具集成**: SSH、Telnet等工具为AI提供"手脚"，类似MCP工具
- **知识库存储**: 本地文件系统（`./doc/`目录）
- **资源管理**: 统一抽象，支持多种资源类型

## 项目方案

本项目基于**方案2**实现：

**用户群体**: 运维/技术支持/研发

**核心价值**: 通过AI自动化运维任务，减少重复性工作，提高问题解决效率。


