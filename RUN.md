# 运行指南

## 快速开始

### 1. 安装依赖

首次运行前需要安装依赖：

```bash
npm install
```

### 2. 运行开发服务器

```bash
npm run tauri dev
```

这个命令会：
- 启动Vite开发服务器（前端）
- 编译Rust代码（后端）
- 启动Tauri应用窗口

### 3. 构建生产版本

```bash
npm run tauri build
```

构建完成后，可执行文件位于：
- macOS: `src-tauri/target/release/bundle/macos/`
- Windows: `src-tauri/target/release/bundle/msi/`
- Linux: `src-tauri/target/release/bundle/`

## 开发模式

开发模式下，前端代码会热重载，修改后自动刷新。

## 注意事项

1. **LLM服务**: 确保局域网Ollama服务可访问（`http://10.35.148.111:11434`）
2. **端口**: 前端开发服务器使用端口 `1420`
3. **首次编译**: Rust代码首次编译可能需要几分钟时间

## 常用命令

```bash
# 仅运行前端开发服务器
npm run dev

# 仅构建前端
npm run build

# 预览前端构建结果
npm run preview

# Tauri相关命令
npm run tauri dev      # 开发模式
npm run tauri build    # 构建生产版本
```

## 故障排查

### 端口被占用
如果端口1420被占用，修改 `vite.config.js` 中的端口号。

### LLM连接失败
检查Ollama服务是否运行：
```bash
curl http://10.35.148.111:11434/api/tags
```

### Rust编译错误
清理并重新编译：
```bash
cd src-tauri
cargo clean
cargo build
```

