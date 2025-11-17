# 多知识库管理架构设计

## 📋 需求概述

支持多个产品/平台的知识库管理，运维工程师可以在不同知识库之间切换，每个知识库包含独立的配置和文档。

## 🏗️ Clean Architecture 设计

### 1. 领域层（Domain Layer）

#### 实体定义

```rust
// src-tauri/src/domain/entities/knowledge_base.rs

pub struct KnowledgeBase {
    pub id: String,              // 知识库ID（目录名）
    pub name: String,            // 显示名称
    pub path: PathBuf,           // 知识库路径（./doc/{id}/）
    pub description: Option<String>, // 描述信息
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl KnowledgeBase {
    pub fn new(id: String, name: String, base_path: &Path) -> Self {
        let path = base_path.join(&id);
        let now = Utc::now();
        Self {
            id,
            name,
            path,
            description: None,
            created_at: now,
            updated_at: now,
        }
    }
}
```

#### 仓储接口

```rust
// src-tauri/src/domain/repositories/knowledge_base_repository.rs

pub trait KnowledgeBaseRepository: Send + Sync {
    /// 列出所有知识库
    fn list(&self) -> Result<Vec<KnowledgeBase>, RepositoryError>;
    
    /// 根据ID获取知识库
    fn get_by_id(&self, id: &str) -> Result<KnowledgeBase, RepositoryError>;
    
    /// 创建新知识库
    fn create(&self, kb: &KnowledgeBase) -> Result<(), RepositoryError>;
    
    /// 删除知识库
    fn delete(&self, id: &str) -> Result<(), RepositoryError>;
    
    /// 更新知识库信息
    fn update(&self, kb: &KnowledgeBase) -> Result<(), RepositoryError>;
    
    /// 获取当前激活的知识库ID
    fn get_current(&self) -> Result<Option<String>, RepositoryError>;
    
    /// 设置当前激活的知识库
    fn set_current(&self, id: &str) -> Result<(), RepositoryError>;
}
```

### 2. 应用层（Application Layer）

#### 用例实现

```rust
// src-tauri/src/application/usecases/knowledge_base/list_knowledge_bases.rs

pub struct ListKnowledgeBasesUseCase {
    repo: Arc<dyn KnowledgeBaseRepository>,
}

impl ListKnowledgeBasesUseCase {
    pub fn new(repo: Arc<dyn KnowledgeBaseRepository>) -> Self {
        Self { repo }
    }
    
    pub fn execute(&self) -> Result<Vec<KnowledgeBase>, UseCaseError> {
        self.repo.list()
            .map_err(|e| UseCaseError::Repository(e))
    }
}
```

```rust
// src-tauri/src/application/usecases/knowledge_base/switch_knowledge_base.rs

pub struct SwitchKnowledgeBaseUseCase {
    repo: Arc<dyn KnowledgeBaseRepository>,
}

impl SwitchKnowledgeBaseUseCase {
    pub fn new(repo: Arc<dyn KnowledgeBaseRepository>) -> Self {
        Self { repo }
    }
    
    pub fn execute(&self, id: &str) -> Result<KnowledgeBase, UseCaseError> {
        // 验证知识库存在
        let kb = self.repo.get_by_id(id)
            .map_err(|e| UseCaseError::Repository(e))?;
        
        // 设置为当前知识库
        self.repo.set_current(id)
            .map_err(|e| UseCaseError::Repository(e))?;
        
        Ok(kb)
    }
}
```

```rust
// src-tauri/src/application/usecases/knowledge_base/create_knowledge_base.rs

pub struct CreateKnowledgeBaseUseCase {
    repo: Arc<dyn KnowledgeBaseRepository>,
}

impl CreateKnowledgeBaseUseCase {
    pub fn new(repo: Arc<dyn KnowledgeBaseRepository>) -> Self {
        Self { repo }
    }
    
    pub fn execute(&self, name: String, description: Option<String>) -> Result<KnowledgeBase, UseCaseError> {
        // 生成ID（基于名称）
        let id = self.generate_id(&name);
        
        // 创建知识库实体
        let base_path = PathBuf::from("./doc");
        let mut kb = KnowledgeBase::new(id.clone(), name, &base_path);
        kb.description = description;
        
        // 保存到仓储
        self.repo.create(&kb)
            .map_err(|e| UseCaseError::Repository(e))?;
        
        // 初始化知识库目录结构
        self.initialize_structure(&kb.path)?;
        
        Ok(kb)
    }
    
    fn generate_id(&self, name: &str) -> String {
        // 将名称转换为ID（小写、连字符）
        name.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .trim_matches('-')
            .to_string()
    }
    
    fn initialize_structure(&self, path: &Path) -> Result<(), UseCaseError> {
        // 创建标准目录结构
        let dirs = vec![
            "company",
            "environments/production",
            "environments/staging",
            "environments/development",
            "services",
            "operations/deployment",
            "operations/monitoring",
            "operations/emergency",
            "standards",
            "history/incidents",
            "history/solutions",
        ];
        
        for dir in dirs {
            std::fs::create_dir_all(path.join(dir))
                .map_err(|e| UseCaseError::IO(e))?;
        }
        
        Ok(())
    }
}
```

### 3. 基础设施层（Infrastructure Layer）

```rust
// src-tauri/src/infrastructure/repositories/file_system_knowledge_base_repository.rs

pub struct FileSystemKnowledgeBaseRepository {
    base_path: PathBuf,
    config_file: PathBuf,
}

impl FileSystemKnowledgeBaseRepository {
    pub fn new(base_path: PathBuf) -> Self {
        let config_file = base_path.join(".knowledge_bases.json");
        Self {
            base_path,
            config_file,
        }
    }
    
    fn load_config(&self) -> Result<KnowledgeBaseConfig, RepositoryError> {
        if !self.config_file.exists() {
            return Ok(KnowledgeBaseConfig::default());
        }
        
        let content = std::fs::read_to_string(&self.config_file)
            .map_err(|e| RepositoryError::IO(e))?;
        
        serde_json::from_str(&content)
            .map_err(|e| RepositoryError::Parse(e))
    }
    
    fn save_config(&self, config: &KnowledgeBaseConfig) -> Result<(), RepositoryError> {
        let content = serde_json::to_string_pretty(config)
            .map_err(|e| RepositoryError::Serialize(e))?;
        
        std::fs::write(&self.config_file, content)
            .map_err(|e| RepositoryError::IO(e))?;
        
        Ok(())
    }
}

impl KnowledgeBaseRepository for FileSystemKnowledgeBaseRepository {
    fn list(&self) -> Result<Vec<KnowledgeBase>, RepositoryError> {
        let config = self.load_config()?;
        let mut result = Vec::new();
        
        // 扫描目录
        if self.base_path.exists() {
            for entry in std::fs::read_dir(&self.base_path)
                .map_err(|e| RepositoryError::IO(e))? {
                let entry = entry.map_err(|e| RepositoryError::IO(e))?;
                let path = entry.path();
                
                if path.is_dir() && !path.file_name().unwrap().to_string_lossy().starts_with('.') {
                    let id = path.file_name().unwrap().to_string_lossy().to_string();
                    
                    // 从配置中获取信息，或使用默认值
                    let kb_info = config.knowledge_bases.get(&id)
                        .cloned()
                        .unwrap_or_else(|| KnowledgeBaseInfo {
                            name: id.clone(),
                            description: None,
                            created_at: Utc::now(),
                            updated_at: Utc::now(),
                        });
                    
                    result.push(KnowledgeBase {
                        id,
                        name: kb_info.name,
                        path,
                        description: kb_info.description,
                        created_at: kb_info.created_at,
                        updated_at: kb_info.updated_at,
                    });
                }
            }
        }
        
        Ok(result)
    }
    
    fn get_by_id(&self, id: &str) -> Result<KnowledgeBase, RepositoryError> {
        let path = self.base_path.join(id);
        
        if !path.exists() {
            return Err(RepositoryError::NotFound(id.to_string()));
        }
        
        let config = self.load_config()?;
        let kb_info = config.knowledge_bases.get(id)
            .cloned()
            .unwrap_or_else(|| KnowledgeBaseInfo {
                name: id.to_string(),
                description: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
        
        Ok(KnowledgeBase {
            id: id.to_string(),
            name: kb_info.name,
            path,
            description: kb_info.description,
            created_at: kb_info.created_at,
            updated_at: kb_info.updated_at,
        })
    }
    
    fn create(&self, kb: &KnowledgeBase) -> Result<(), RepositoryError> {
        // 创建目录
        std::fs::create_dir_all(&kb.path)
            .map_err(|e| RepositoryError::IO(e))?;
        
        // 更新配置
        let mut config = self.load_config()?;
        config.knowledge_bases.insert(kb.id.clone(), KnowledgeBaseInfo {
            name: kb.name.clone(),
            description: kb.description.clone(),
            created_at: kb.created_at,
            updated_at: kb.updated_at,
        });
        self.save_config(&config)?;
        
        Ok(())
    }
    
    fn delete(&self, id: &str) -> Result<(), RepositoryError> {
        let path = self.base_path.join(id);
        
        if !path.exists() {
            return Err(RepositoryError::NotFound(id.to_string()));
        }
        
        // 删除目录
        std::fs::remove_dir_all(&path)
            .map_err(|e| RepositoryError::IO(e))?;
        
        // 更新配置
        let mut config = self.load_config()?;
        config.knowledge_bases.remove(id);
        
        // 如果删除的是当前知识库，清除当前设置
        if config.current_knowledge_base.as_ref() == Some(&id.to_string()) {
            config.current_knowledge_base = None;
        }
        
        self.save_config(&config)?;
        
        Ok(())
    }
    
    fn update(&self, kb: &KnowledgeBase) -> Result<(), RepositoryError> {
        let mut config = self.load_config()?;
        
        if let Some(info) = config.knowledge_bases.get_mut(&kb.id) {
            info.name = kb.name.clone();
            info.description = kb.description.clone();
            info.updated_at = Utc::now();
            self.save_config(&config)?;
            Ok(())
        } else {
            Err(RepositoryError::NotFound(kb.id.clone()))
        }
    }
    
    fn get_current(&self) -> Result<Option<String>, RepositoryError> {
        let config = self.load_config()?;
        Ok(config.current_knowledge_base.clone())
    }
    
    fn set_current(&self, id: &str) -> Result<(), RepositoryError> {
        // 验证知识库存在
        self.get_by_id(id)?;
        
        let mut config = self.load_config()?;
        config.current_knowledge_base = Some(id.to_string());
        self.save_config(&config)?;
        
        Ok(())
    }
}
```

### 4. 表示层（Presentation Layer）

#### UI组件设计

```html
<!-- 顶部工具栏 - 知识库切换器 -->
<div class="knowledge-base-selector">
  <label>知识库:</label>
  <select v-model="currentKnowledgeBaseId" @change="switchKnowledgeBase">
    <option v-for="kb in knowledgeBases" :key="kb.id" :value="kb.id">
      {{ kb.name }}
    </option>
  </select>
  <button @click="showCreateDialog">+ 新建</button>
</div>
```

## 📁 目录结构

```
src-tauri/src/
├── domain/
│   ├── entities/
│   │   └── knowledge_base.rs          # 知识库实体
│   └── repositories/
│       └── knowledge_base_repository.rs  # 仓储接口
├── application/
│   └── usecases/
│       └── knowledge_base/
│           ├── list_knowledge_bases.rs
│           ├── switch_knowledge_base.rs
│           ├── create_knowledge_base.rs
│           └── delete_knowledge_base.rs
└── infrastructure/
    └── repositories/
        └── file_system_knowledge_base_repository.rs  # 文件系统实现
```

## 🔄 工作流程

1. **启动应用**
   - 加载所有知识库列表
   - 读取当前激活的知识库
   - 如果未设置，使用第一个知识库或提示用户选择

2. **切换知识库**
   - 用户从下拉菜单选择知识库
   - 调用SwitchKnowledgeBaseUseCase
   - 更新当前知识库配置
   - 重新加载知识库内容
   - AI对话上下文切换到新知识库

3. **创建知识库**
   - 用户点击"新建"按钮
   - 输入知识库名称和描述
   - 调用CreateKnowledgeBaseUseCase
   - 创建目录结构
   - 自动切换到新知识库

4. **删除知识库**
   - 用户选择删除操作
   - 确认对话框
   - 调用DeleteKnowledgeBaseUseCase
   - 删除目录和配置

## 📝 配置文件格式

```json
// .doc/.knowledge_bases.json
{
  "current_knowledge_base": "product-a",
  "knowledge_bases": {
    "product-a": {
      "name": "产品A",
      "description": "产品A的运维知识库",
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-15T10:30:00Z"
    },
    "product-b": {
      "name": "产品B",
      "description": "产品B的运维知识库",
      "created_at": "2024-01-05T00:00:00Z",
      "updated_at": "2024-01-20T14:20:00Z"
    }
  }
}
```

## 🎯 实现优先级

### Phase 1 - MVP
- [ ] 知识库实体和仓储接口定义
- [ ] 文件系统仓储实现
- [ ] 列出知识库用例
- [ ] 切换知识库用例
- [ ] UI知识库切换器

### Phase 2 - 增强
- [ ] 创建知识库用例
- [ ] 删除知识库用例
- [ ] 知识库信息编辑
- [ ] 知识库导入/导出

### Phase 3 - 高级
- [ ] 知识库模板
- [ ] 知识库同步
- [ ] 知识库权限管理

