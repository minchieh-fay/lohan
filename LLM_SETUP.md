# LLM服务配置说明

## 架构设计

LLM服务采用Clean Architecture设计，位于基础设施层：

```
domain/services/llm_service.rs          # LLM服务接口（领域层）
  ↓
infrastructure/llm/ollama_client.rs     # Ollama客户端实现（基础设施层）
  ↓
application/usecases/llm/chat_with_ai.rs # 聊天用例（应用层）
```

## 当前配置

- **服务地址**: `http://10.35.148.111:11434`
- **协议**: Ollama API
- **默认模型**: `qwen`

## Tauri命令

### 1. chat_with_ai
与AI进行对话

```typescript
import { invoke } from '@tauri-apps/api/tauri';

const response = await invoke('chat_with_ai', {
  message: '你好',
  context: '当前选中的资源：Web-01, Web-02'
});
```

### 2. check_llm_health
检查LLM服务是否可用

```typescript
const isHealthy = await invoke('check_llm_health');
```

### 3. list_llm_models
列出可用的模型列表

```typescript
const models = await invoke('list_llm_models');
```

## 使用示例

### 前端调用示例

```javascript
// 检查LLM服务健康状态
async function checkLLM() {
  try {
    const isHealthy = await window.__TAURI__.invoke('check_llm_health');
    console.log('LLM服务状态:', isHealthy ? '正常' : '异常');
  } catch (error) {
    console.error('LLM服务不可用:', error);
  }
}

// 发送消息给AI
async function sendMessage(userMessage, selectedResources) {
  try {
    const context = selectedResources.length > 0 
      ? `已选择资源：${selectedResources.join(', ')}`
      : null;
    
    const response = await window.__TAURI__.invoke('chat_with_ai', {
      message: userMessage,
      context: context
    });
    
    return response;
  } catch (error) {
    console.error('AI响应失败:', error);
    throw error;
  }
}
```

## 修改配置

如需修改Ollama服务地址或模型，编辑 `src-tauri/src/infrastructure/llm/ollama_client.rs`:

```rust
pub fn from_config() -> Self {
    Self::new(
        "http://your-ollama-server:11434".to_string(),  // 修改地址
        "your-model-name".to_string(),                  // 修改模型名
    )
}
```

## 扩展支持

如需支持其他LLM服务（如OpenAI、Claude等），只需：

1. 实现 `LLMService` trait
2. 在基础设施层创建新的客户端
3. 通过依赖注入使用新的实现

