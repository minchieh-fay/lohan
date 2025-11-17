# MCP模块设计文档

## 问题解答

### 问题1：一个大MCP还是多个？

**建议：多个MCP工具，按功能分类**

**分类方案：**

1. **通用工具模块**（Common Tools）
   - 获取当前时间
   - 文件操作（读取、写入、列表）
   - 系统信息（CPU、内存、磁盘）
   - 文本处理（搜索、替换、格式化）

2. **SSH工具模块**（SSH Tools）
   - SSH命令执行
   - 文件传输
   - 远程文件操作
   - 远程系统信息获取

3. **分析工具模块**（Analysis Tools）
   - 网络包分析（pcap）
   - Coredump分析
   - 日志分析
   - 性能分析

4. **运维工具模块**（Operations Tools）
   - 服务管理（启动、停止、重启）
   - 进程管理
   - 监控数据收集
   - 告警处理

**优势：**
- 模块化，易于维护和扩展
- 可以按需加载，减少资源占用
- 每个模块职责单一，符合单一职责原则

### 问题2：MCP插件放哪里？

**建议：基础设施层（Infrastructure Layer）**

**架构设计：**

```
domain/
  └── services/
      └── mcp_tool.rs          # MCP工具接口定义（领域层）

infrastructure/
  └── mcp/
      ├── tools/                # MCP工具实现（基础设施层）
      │   ├── common/          # 通用工具
      │   │   ├── get_time.rs
      │   │   ├── file_ops.rs
      │   │   └── system_info.rs
      │   ├── ssh/             # SSH工具（依赖SSH连接器）
      │   │   ├── ssh_exec.rs
      │   │   └── ssh_file_transfer.rs
      │   └── analysis/        # 分析工具
      │       ├── pcap_analyzer.rs
      │       └── coredump_analyzer.rs
      └── mcp_manager.rs       # MCP工具管理器
```

**依赖关系：**
- SSH工具可以依赖 `infrastructure::connectors::ssh_connector`
- 通用工具不依赖其他模块
- 分析工具可以依赖文件操作工具

### 问题3：怎么写MCP模块？

**MCP工具接口设计：**

```rust
pub trait MCPTool: Send + Sync {
    /// 工具名称
    fn name(&self) -> &str;
    
    /// 工具描述
    fn description(&self) -> &str;
    
    /// 工具参数定义
    fn parameters(&self) -> serde_json::Value;
    
    /// 执行工具
    async fn execute(&self, arguments: serde_json::Value) -> Result<serde_json::Value, MCPToolError>;
}
```

**示例：获取本机时间工具**

```rust
pub struct GetTimeTool;

impl MCPTool for GetTimeTool {
    fn name(&self) -> &str {
        "get_current_time"
    }
    
    fn description(&self) -> &str {
        "获取当前系统时间，支持指定时区和格式"
    }
    
    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "timezone": {
                    "type": "string",
                    "description": "时区，例如：Asia/Shanghai，默认为系统时区"
                },
                "format": {
                    "type": "string",
                    "description": "时间格式，例如：RFC3339, ISO8601，默认为RFC3339"
                }
            }
        })
    }
    
    async fn execute(&self, arguments: serde_json::Value) -> Result<serde_json::Value, MCPToolError> {
        // 实现获取时间的逻辑
        // ...
    }
}
```

## 实现计划

1. ✅ 定义MCP工具接口（领域层）
2. ✅ 实现工具管理器（基础设施层）
3. ✅ 实现获取本机时间工具（示例）
4. ✅ 实现SSH命令执行工具（示例）
5. ⏳ 集成到AI对话流程中（下一步）

## 已实现的工具

### 1. GetTimeTool（获取本机时间）
- **名称**: `get_current_time`
- **描述**: 获取当前系统时间，支持指定时区和时间格式
- **参数**:
  - `timezone`: 时区（local, UTC等）
  - `format`: 时间格式（RFC3339, timestamp, human）
- **返回**: JSON格式的时间信息

### 2. SSHExecTool（SSH命令执行）
- **名称**: `ssh_execute_command`
- **描述**: 在远程SSH服务器上执行命令
- **参数**:
  - `host`: SSH服务器IP地址
  - `port`: SSH端口（默认22）
  - `username`: SSH用户名
  - `password`: SSH密码
  - `command`: 要执行的命令
- **返回**: 命令执行结果

## Tauri命令

### list_mcp_tools
列出所有可用的MCP工具

```typescript
const tools = await invoke('list_mcp_tools');
```

### execute_mcp_tool
执行指定的MCP工具

```typescript
const result = await invoke('execute_mcp_tool', {
  tool_name: 'get_current_time',
  arguments: {
    timezone: 'local',
    format: 'human'
  }
});
```

## 目录结构

```
src-tauri/src/
├── domain/
│   └── services/
│       └── mcp_tool.rs          # MCP工具接口定义
├── infrastructure/
│   └── mcp/
│       ├── tools/
│       │   ├── common/          # 通用工具
│       │   │   └── get_time.rs
│       │   └── ssh_tools/       # SSH工具
│       │       └── ssh_exec.rs
│       └── mcp_manager.rs       # 工具管理器
```

## 添加新工具的步骤

1. 在 `infrastructure/mcp/tools/` 下创建工具文件
2. 实现 `MCPTool` trait
3. 在 `MCPManager::register_tools()` 中注册工具
4. 完成！

