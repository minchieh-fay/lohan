#!/bin/bash

# Lohan 启动脚本 - 使用国内镜像
# 设置 HF_ENDPOINT 环境变量使用 Hugging Face 国内镜像

echo "使用国内镜像启动 Lohan..."
echo "镜像地址: https://hf-mirror.com"

export HF_ENDPOINT="https://hf-mirror.com"

wails dev

