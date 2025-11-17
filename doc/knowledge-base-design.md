# AI运维助手 - 知识库设计方案

## 📚 为什么需要知识库？

### 问题场景
```
❌ 没有知识库的情况：
用户: "帮我重启应用服务"
AI: "请问你们的应用服务名称是什么？部署在哪些服务器上？"
用户: "tomcat，在/opt/tomcat目录，三台Web服务器上"
AI: "启动命令是什么？"
用户: "sh /opt/tomcat/bin/startup.sh"
...（每次都要重复）

✅ 有知识库的情况：
用户: "帮我重启应用服务"
AI: "好的，我看到知识库中记录：
     - 服务名: Tomcat 9.0
     - 位置: /opt/tomcat
     - 服务器: Web-01, Web-02, Web-03
     - 启动命令: sh /opt/tomcat/bin/startup.sh
     正在执行重启..."
```

---

## 🗂️ 知识库目录结构

```
knowledge-base/
├── company/                          # 公司级配置
│   ├── basic-info.yaml              # 基本信息
│   ├── servers.yaml                 # 服务器清单
│   └── contacts.yaml                # 联系人信息
│
├── environments/                     # 环境配置
│   ├── production/                  # 生产环境
│   │   ├── servers.yaml            # 服务器列表
│   │   ├── services.yaml           # 服务配置
│   │   ├── ports.yaml              # 端口映射
│   │   └── paths.yaml              # 路径规范
│   ├── staging/                     # 预发布环境
│   └── development/                 # 开发环境
│
├── services/                         # 服务知识库
│   ├── web-app/                     # Web应用
│   │   ├── service.yaml            # 服务定义
│   │   ├── deployment.yaml         # 部署规范
│   │   ├── troubleshooting.yaml    # 故障排查手册
│   │   └── monitoring.yaml         # 监控指标
│   ├── database/
│   ├── cache/
│   └── message-queue/
│
├── operations/                       # 运维操作手册
│   ├── deployment/                  # 部署流程
│   │   ├── app-deployment.md       # 应用部署
│   │   ├── db-migration.md         # 数据库迁移
│   │   └── rollback.md             # 回滚流程
│   ├── monitoring/                  # 监控
│   │   ├── alerts.yaml             # 告警规则
│   │   └── metrics.yaml            # 监控指标
│   └── emergency/                   # 应急预案
│       ├── high-load.md            # 高负载处理
│       ├── service-down.md         # 服务宕机
│       └── data-loss.md            # 数据丢失
│
├── standards/                        # 规范标准
│   ├── naming.yaml                  # 命名规范
│   ├── directory-structure.yaml     # 目录结构
│   ├── port-allocation.yaml         # 端口分配
│   └── security-policy.yaml         # 安全策略
│
└── history/                          # 历史记录
    ├── incidents/                   # 事故记录
    └── solutions/                   # 解决方案库
```

---

## 📝 知识库文件格式示例

### 1. 公司基本信息 (`company/basic-info.yaml`)

```yaml
company:
  name: "某某科技有限公司"
  domain: "example.com"
  timezone: "Asia/Shanghai"
  
default_settings:
  shell: "/bin/bash"
  user: "admin"
  sudo_required: true
  
common_directories:
  logs: "/opt/logs"
  applications: "/opt/apps"
  backups: "/data/backups"
  configs: "/etc/apps"
  
common_ports:
  http: 8080
  https: 9443
  admin: 8888
  monitoring: 9090
```

### 2. 环境配置 (`environments/production/paths.yaml`)

```yaml
environment: production

directories:
  # 应用目录
  app_root: "/opt/apps"
  app_logs: "/opt/logs"
  app_config: "/etc/apps/config"
  
  # 数据目录
  data_root: "/data"
  backups: "/data/backups"
  uploads: "/data/uploads"
  
  # 临时目录
  temp: "/tmp/apps"
  cache: "/var/cache/apps"

ports:
  web_app: 9443
  admin_panel: 8888
  metrics: 9090
  health_check: 8081

urls:
  app_domain: "app.example.com"
  admin_domain: "admin.example.com"
  api_domain: "api.example.com"
```

### 3. 服务定义 (`services/web-app/service.yaml`)

```yaml
service:
  name: "Web应用"
  type: "tomcat"
  version: "9.0.65"
  
deployment:
  servers:
    - Web-01
    - Web-02
    - Web-03
  
  paths:
    install_dir: "/opt/apps/tomcat"
    logs_dir: "/opt/logs/tomcat"
    config_dir: "/opt/apps/tomcat/conf"
    pid_file: "/opt/apps/tomcat/tomcat.pid"
  
  ports:
    http: 9443
    ajp: 8009
    shutdown: 8005
  
  commands:
    start: "sh /opt/apps/tomcat/bin/startup.sh"
    stop: "sh /opt/apps/tomcat/bin/shutdown.sh"
    restart: "sh /opt/apps/tomcat/bin/shutdown.sh && sh /opt/apps/tomcat/bin/startup.sh"
    status: "ps aux | grep tomcat | grep -v grep"
    check_health: "curl -s http://localhost:9443/health"
  
  monitoring:
    health_check_url: "http://localhost:9443/health"
    health_check_interval: 30
    log_file: "/opt/logs/tomcat/catalina.out"
    error_patterns:
      - "OutOfMemoryError"
      - "Exception"
      - "ERROR"
  
  dependencies:
    - "JDK 11"
    - "MySQL 8.0"
    - "Redis 6.2"
```

### 4. 故障排查手册 (`services/web-app/troubleshooting.yaml`)

```yaml
troubleshooting:
  - symptom: "服务无法启动"
    possible_causes:
      - "端口被占用"
      - "配置文件错误"
      - "权限不足"
    diagnostic_steps:
      - command: "netstat -tulpn | grep 9443"
        description: "检查端口是否被占用"
      - command: "cat /opt/logs/tomcat/catalina.out | tail -100"
        description: "查看启动日志"
      - command: "ls -la /opt/apps/tomcat"
        description: "检查文件权限"
    solutions:
      - "kill占用端口的进程"
      - "检查conf/server.xml配置"
      - "chown -R admin:admin /opt/apps/tomcat"
  
  - symptom: "响应缓慢"
    possible_causes:
      - "内存不足"
      - "数据库慢查询"
      - "网络延迟"
    diagnostic_steps:
      - command: "free -h"
        description: "检查内存使用"
      - command: "top -p $(cat /opt/apps/tomcat/tomcat.pid)"
        description: "查看进程资源占用"
      - command: "tail -1000 /opt/logs/tomcat/catalina.out | grep 'slow query'"
        description: "查找慢查询"
    solutions:
      - "重启应用释放内存"
      - "优化数据库索引"
      - "检查网络连接"
```

### 5. 部署流程 (`operations/deployment/app-deployment.md`)

```markdown
# Web应用部署流程

## 前置条件
- [ ] 代码已通过所有测试
- [ ] 数据库脚本已准备
- [ ] 配置文件已更新
- [ ] 已通知相关人员

## 部署步骤

### 1. 备份当前版本
```bash
cd /opt/apps
tar -czf tomcat-backup-$(date +%Y%m%d%H%M%S).tar.gz tomcat/
mv tomcat-backup-*.tar.gz /data/backups/
```

### 2. 停止服务
```bash
sh /opt/apps/tomcat/bin/shutdown.sh
# 等待30秒确认进程退出
sleep 30
ps aux | grep tomcat | grep -v grep && kill -9 $(cat /opt/apps/tomcat/tomcat.pid)
```

### 3. 部署新版本
```bash
# 上传war包
scp app-v1.2.3.war admin@Web-01:/opt/apps/tomcat/webapps/

# 解压并设置权限
cd /opt/apps/tomcat/webapps/
unzip -o app-v1.2.3.war -d app/
chown -R admin:admin app/
```

### 4. 更新配置
```bash
cp /etc/apps/config/application.properties /opt/apps/tomcat/webapps/app/WEB-INF/classes/
```

### 5. 启动服务
```bash
sh /opt/apps/tomcat/bin/startup.sh
```

### 6. 健康检查
```bash
# 等待服务启动
sleep 30

# 检查进程
ps aux | grep tomcat | grep -v grep

# 检查端口
netstat -tulpn | grep 9443

# 健康检查
curl -s http://localhost:9443/health | jq
```

## 验证清单
- [ ] 服务正常启动
- [ ] 健康检查通过
- [ ] 日志无ERROR
- [ ] 功能测试通过

## 回滚方案
如果部署失败，执行：
```bash
sh /opt/apps/tomcat/bin/shutdown.sh
rm -rf /opt/apps/tomcat
cd /data/backups
tar -xzf tomcat-backup-最新版本.tar.gz -C /opt/apps/
sh /opt/apps/tomcat/bin/startup.sh
```
```

---

## 🤖 AI如何使用知识库

### 工作流程

```
用户输入 → AI解析意图 → 查询知识库 → 生成执行计划 → 执行 → 反馈结果
                              ↓
                    找到相关配置、路径、命令
```

### 示例对话

#### 场景1：重启服务
```
用户: "重启Web应用"

AI内部处理:
1. 识别意图: restart_service
2. 查询 services/web-app/service.yaml
   - 找到服务器: Web-01, Web-02, Web-03
   - 找到重启命令: sh /opt/apps/tomcat/bin/shutdown.sh && ...
3. 查询 operations/deployment/app-deployment.md
   - 检查是否需要备份
   - 查看健康检查步骤

AI回复:
"我将重启Web应用（Tomcat 9.0），部署在3台服务器上。
根据知识库，重启流程包括：
1. 优雅停止服务
2. 等待30秒
3. 启动服务
4. 健康检查

是否立即执行？"
```

#### 场景2：诊断问题
```
用户: "Web-02响应很慢，帮我看看"

AI内部处理:
1. 识别意图: diagnose_performance
2. 查询 services/web-app/troubleshooting.yaml
   - 找到"响应缓慢"的排查步骤
3. 查询 services/web-app/service.yaml
   - 获取日志路径: /opt/logs/tomcat/catalina.out
   - 获取监控指标位置
4. 执行诊断命令

AI回复:
"正在按知识库中的故障排查手册诊断...

✓ 内存检查: 使用率78%，偏高
✓ CPU检查: 23%，正常
✓ 日志分析: 发现127个慢查询记录
✓ 网络检查: 正常

根据知识库的解决方案，建议：
1. 重启应用释放内存
2. 优化数据库慢查询
3. 考虑扩容内存

需要我执行哪项？"
```

#### 场景3：部署新版本
```
用户: "部署Web应用v1.2.3"

AI内部处理:
1. 识别意图: deploy_application
2. 查询 operations/deployment/app-deployment.md
   - 获取完整部署流程
3. 查询 services/web-app/service.yaml
   - 确认部署服务器
   - 确认部署路径

AI回复:
"我将按照知识库中的标准部署流程进行：

前置检查：
✓ 当前版本: v1.2.2
✓ 目标版本: v1.2.3
✓ 部署服务器: Web-01, Web-02, Web-03

部署流程：
1. 备份当前版本 → /data/backups/
2. 停止服务（优雅停机）
3. 部署war包到 /opt/apps/tomcat/webapps/
4. 更新配置文件
5. 启动服务
6. 健康检查

预计耗时: 5-8分钟
是否继续？"
```

---

## 🔧 知识库管理功能

### 1. 界面设计

```
┌─────────────────────────────────────────┐
│  知识库管理                              │
├─────────────────────────────────────────┤
│                                          │
│  📁 company/                            │
│     ├─ basic-info.yaml         [编辑]   │
│     ├─ servers.yaml            [编辑]   │
│     └─ contacts.yaml           [编辑]   │
│                                          │
│  📁 environments/                        │
│     ├─ 📁 production/                   │
│     │    ├─ servers.yaml       [编辑]   │
│     │    ├─ services.yaml      [编辑]   │
│     │    ├─ ports.yaml         [编辑]   │
│     │    └─ paths.yaml         [编辑]   │
│     ├─ 📁 staging/                      │
│     └─ 📁 development/                  │
│                                          │
│  📁 services/                            │
│     ├─ 📁 web-app/                      │
│     │    ├─ service.yaml       [编辑]   │
│     │    ├─ troubleshooting... [编辑]   │
│     │    └─ monitoring.yaml    [编辑]   │
│     ├─ 📁 database/                     │
│     └─ 📁 cache/                        │
│                                          │
│  [+ 新建文件夹] [+ 新建文件]            │
│  [导入] [导出] [同步到云端]             │
└─────────────────────────────────────────┘
```

### 2. 编辑器功能

- **YAML/Markdown 编辑器**
- **语法高亮和验证**
- **模板快速生成**
- **版本历史**
- **AI辅助填写**（让AI帮你生成配置）

### 3. 导入导出

```bash
# 导出知识库
AI运维助手 → 知识库 → 导出
→ knowledge-base-export-20241117.zip

# 导入知识库
AI运维助手 → 知识库 → 导入
→ 选择 .zip 文件
```

### 4. 团队协作

- **云端同步**：知识库自动同步到团队
- **权限控制**：不同角色看到不同的知识
- **变更审批**：重要配置需要审批才能修改

---

## 📋 知识库最佳实践

### 1. 命名规范

| 类型 | 规范 | 示例 |
|------|------|------|
| 文件名 | 小写+连字符 | `web-app-service.yaml` |
| 服务名 | 业务名称 | `用户服务`, `订单服务` |
| 服务器名 | 类型-序号 | `Web-01`, `DB-Master` |
| 路径 | 绝对路径 | `/opt/apps/tomcat` |

### 2. 分层原则

```
公司级 (company/)     → 所有环境共享
  ↓
环境级 (environments/) → 特定环境配置
  ↓
服务级 (services/)    → 具体服务细节
  ↓
操作级 (operations/)  → 运维流程
```

### 3. 更新频率

| 知识类型 | 更新频率 | 触发时机 |
|----------|----------|----------|
| 基本信息 | 很少 | 公司信息变更 |
| 服务器清单 | 中等 | 增减服务器 |
| 服务配置 | 频繁 | 版本升级 |
| 故障手册 | 累积 | 遇到新问题 |
| 部署流程 | 中等 | 流程优化 |

### 4. 质量检查

**AI会自动检查知识库：**
- ✓ YAML格式是否正确
- ✓ 路径是否存在
- ✓ 端口是否冲突
- ✓ 命令是否有效
- ✓ 文档是否过时

---

## 🚀 实现优先级

### Phase 1 - MVP (必须)
- [x] 基本的YAML配置解析
- [x] 服务器、路径、端口配置
- [x] 简单的知识库查询
- [x] 文件编辑器

### Phase 2 - 增强 (重要)
- [ ] 完整的目录结构
- [ ] 故障排查手册
- [ ] 部署流程文档
- [ ] 导入导出功能
- [ ] AI辅助生成配置

### Phase 3 - 高级 (加分)
- [ ] 版本历史
- [ ] 团队协作
- [ ] 云端同步
- [ ] 智能推荐
- [ ] 知识图谱

---

## 💡 AI智能特性

### 1. 自动学习
```
AI观察到你每次都在 /opt/logs 找日志
→ 自动建议添加到知识库
→ "检测到你经常访问 /opt/logs，要不要把它加入知识库？"
```

### 2. 知识补全
```
用户创建了 web-app/service.yaml
但没有创建 troubleshooting.yaml

AI提示: "我发现你定义了Web应用服务，需要我帮你生成故障排查手册吗？"
```

### 3. 一致性检查
```
知识库说日志在 /opt/logs
但AI连接服务器发现日志在 /var/log/apps

AI警告: "知识库信息可能过时，实际日志路径是 /var/log/apps，需要更新吗？"
```

### 4. 智能推荐
```
用户: "Web应用挂了"

AI查询知识库，发现有详细的troubleshooting.yaml
→ 自动按手册步骤诊断
→ 而不是乱猜
```

---

## 📊 价值体现

| 场景 | 没有知识库 | 有知识库 |
|------|-----------|---------|
| 重启服务 | 5分钟（需要问路径、命令） | 30秒（AI直接执行） |
| 诊断问题 | 10分钟（需要问日志位置） | 2分钟（自动按手册排查） |
| 新人培训 | 1周（需要老员工教） | 1天（AI根据知识库指导） |
| 故障处理 | 依赖经验（可能遗漏） | 按手册流程（不会遗漏） |

**知识库让AI从"聊天工具"变成"领域专家"！**

