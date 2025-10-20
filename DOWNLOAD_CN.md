# 国内下载模型指南

## 方法一：使用启动脚本（推荐）

直接运行国内镜像启动脚本：

```bash
./run-cn.sh
```

## 方法二：设置环境变量

在终端中设置环境变量后启动：

```bash
# macOS/Linux
export HF_ENDPOINT="https://hf-mirror.com"
wails dev

# Windows PowerShell
$env:HF_ENDPOINT="https://hf-mirror.com"
wails dev

# Windows CMD
set HF_ENDPOINT=https://hf-mirror.com
wails dev
```

## 方法三：手动下载

如果自动下载失败，可以手动下载模型文件：

### 1. Qwen2.5-Coder-1.5B (1GB) - 推荐
```bash
wget https://hf-mirror.com/Qwen/Qwen2.5-Coder-1.5B-Instruct-GGUF/resolve/main/qwen2.5-coder-1.5b-instruct-q4_k_m.gguf
mv qwen2.5-coder-1.5b-instruct-q4_k_m.gguf ./models/
```

### 2. Qwen2.5-Coder-3B (2GB)
```bash
wget https://hf-mirror.com/Qwen/Qwen2.5-Coder-3B-Instruct-GGUF/resolve/main/qwen2.5-coder-3b-instruct-q4_k_m.gguf
mv qwen2.5-coder-3b-instruct-q4_k_m.gguf ./models/
```

### 3. Qwen2.5-Coder-7B (4.5GB)
```bash
wget https://hf-mirror.com/Qwen/Qwen2.5-Coder-7B-Instruct-GGUF/resolve/main/qwen2.5-coder-7b-instruct-q4_k_m.gguf
mv qwen2.5-coder-7b-instruct-q4_k_m.gguf ./models/
```

## 国内镜像源

推荐使用以下国内镜像源：

1. **hf-mirror.com** (推荐)
   - 地址：https://hf-mirror.com
   - 速度快，稳定

2. **ModelScope** (阿里云)
   - 地址：https://modelscope.cn
   - 需要在网站上搜索对应模型

## 验证下载

模型文件应该下载到 `./models/` 目录下，文件名格式为 `*.gguf`

检查文件是否下载完整：

```bash
ls -lh ./models/
```

确保文件大小正确：
- Qwen2.5-Coder-1.5B: 约 1GB
- Qwen2.5-Coder-3B: 约 2GB
- Qwen2.5-Coder-7B: 约 4.5GB

