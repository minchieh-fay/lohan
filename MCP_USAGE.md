# MCP工具使用指南

## 问题解答总结

### 问题1：一个大MCP还是多个？

**答案：多个MCP工具，按功能分类**

- **通用工具模块**：获取时间、文件操作等
- **SSH工具模块**：远程命令执行、文件传输等
- **分析工具模块**：pcap分析、coredump分析等
- **运维工具模块**：服务管理、进程管理等

### 问题2：MCP插件放哪里？

**答案：基础设施层（Infrastructure Layer）**

- 领域层定义接口（`domain/services/mcp_tool.rs`）
- 基础设施层实现工具（`infrastructure/mcp/tools/`）
- SSH工具可以依赖SSH连接器（`infrastructure::connectors::ssh_connector`）

### 问题3：怎么写MCP模块？

**答案：实现 `MCPTool` trait**

## 示例：获取本机时间工具

### 实现代码

```rust
use async_trait::async_trait;
use chrono::{Local, Utc};
use serde_json::{json, Value};
use crate::domain::services::mcp_tool::{MCPTool, MCPToolError};

pub struct GetTimeTool;

#[async_trait]
impl MCPTool for GetTimeTool {
    fn name(&self) -> &str {
        "get_current_time"
    }
    
    fn description(&self) -> &str {
        "获取当前系统时间。可以指定时区和时间格式。"
    }
    
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "timezone": {
                    "type": "string",
                    "description": "时区名称，例如：local, UTC",
                    "enum": ["local", "UTC"]
                },
                "format": {
                    "type": "string",
                    "description": "时间格式：RFC3339, timestamp, human",
                    "enum": ["RFC3339", "timestamp", "human"]
                }
            }
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<Value, MCPToolError> {
        // 实现逻辑...
        Ok(json!({"time": "..."}))
    }
}
```

### 使用示例

```typescript
// 列出所有工具
const tools = await invoke('list_mcp_tools');
console.log('可用工具:', tools);

// 执行获取时间工具
const result = await invoke('execute_mcp_tool', {
  tool_name: 'get_current_time',
  arguments: {
    timezone: 'local',
    format: 'human'
  }
});
console.log('当前时间:', result);
```

## 测试工具

在浏览器控制台中测试：

```javascript
// 列出工具
await window.__TAURI__.invoke('list_mcp_tools')

// 获取当前时间
await window.__TAURI__.invoke('execute_mcp_tool', {
  tool_name: 'get_current_time',
  arguments: {
    timezone: 'local',
    format: 'human'
  }
})
```

