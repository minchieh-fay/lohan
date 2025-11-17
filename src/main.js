import './style.css'
import { invoke } from '@tauri-apps/api/tauri'

// 应用状态
let resources = []
let selectedResourceIds = []
let currentResourceType = 'ssh'

// 初始化应用
function initApp() {
  renderApp()
  setupEventListeners()
  checkLLMHealth()
}

// 渲染主界面
function renderApp() {
  document.querySelector('#app').innerHTML = `
    <div class="app">
      <header class="header">
        <h1>Lohan</h1>
        <div class="status">
          <span class="status-indicator offline" id="llmStatus">● AI未加载</span>
        </div>
      </header>
      
      <div class="main-container">
        <aside class="resource-list">
          <div class="resource-list-header">
            <h2>资源列表</h2>
            <button class="btn-add" id="addResourceBtn">+ 添加</button>
          </div>
          <div class="resources" id="resourcesList">
            ${renderResources()}
          </div>
        </aside>
        
        <main class="chat-area">
          <div class="chat-header">
            <div class="selected-info" id="selectedInfo">
              <span>请选择资源</span>
            </div>
          </div>
          <div class="messages-container" id="messagesContainer">
            <div class="message-item ai">
              <div class="message-header">
                <span class="message-role">AI</span>
                <span class="message-time">${new Date().toLocaleTimeString()}</span>
              </div>
              <div class="message-content">
                你好！我是AI运维助手。请先添加资源，然后选择资源进行操作。
              </div>
            </div>
          </div>
          <div class="chat-input">
            <textarea id="chatInput" placeholder="输入你的问题..." rows="1" ${selectedResourceIds.length === 0 ? 'disabled' : ''}></textarea>
            <button id="sendBtn" ${selectedResourceIds.length === 0 ? 'disabled' : ''}>发送</button>
          </div>
        </main>
      </div>
      
      <!-- 添加资源对话框 -->
      <div class="modal-overlay" id="addResourceModal" style="display: none;">
        <div class="modal-content" onclick="event.stopPropagation()">
          <h2>添加资源</h2>
          
          <div class="form-group">
            <label>资源类型</label>
            <select id="resourceTypeSelect" class="form-control">
              <option value="ssh">🖥️ SSH服务器</option>
              <option value="rancher">🐄 Rancher平台</option>
              <option value="kubernetes">☸️ Kubernetes集群</option>
            </select>
          </div>
          
          <!-- SSH配置 -->
          <div id="sshConfig" class="resource-config">
            <div class="form-group">
              <label>IP地址</label>
              <input type="text" id="sshHost" class="form-control" placeholder="例如: 192.168.1.100" />
            </div>
            <div class="form-group">
              <label>SSH端口</label>
              <input type="number" id="sshPort" class="form-control" value="22" />
            </div>
            <div class="form-group">
              <label>用户名</label>
              <input type="text" id="sshUsername" class="form-control" placeholder="例如: root" />
            </div>
            <div class="form-group">
              <label>密码</label>
              <input type="password" id="sshPassword" class="form-control" placeholder="SSH密码" />
            </div>
            
            <!-- 提权配置（非root用户） -->
            <div id="privilegeConfig" style="display: none;">
              <div class="form-group">
                <label>提权方式</label>
                <select id="privilegeMethod" class="form-control">
                  <option value="sudo">sudo</option>
                  <option value="sudo-i">sudo -i</option>
                  <option value="su">su</option>
                </select>
              </div>
              <div class="form-group" id="suPasswordGroup" style="display: none;">
                <label>Root密码</label>
                <input type="password" id="suPassword" class="form-control" placeholder="Root密码（使用su时需要）" />
              </div>
            </div>
          </div>
          
          <!-- Rancher配置 -->
          <div id="rancherConfig" class="resource-config" style="display: none;">
            <div class="form-group">
              <label>Rancher地址</label>
              <input type="text" id="rancherUrl" class="form-control" placeholder="https://rancher.example.com" />
            </div>
            <div class="form-group">
              <label>用户名</label>
              <input type="text" id="rancherUsername" class="form-control" placeholder="Rancher用户名" />
            </div>
            <div class="form-group">
              <label>密码</label>
              <input type="password" id="rancherPassword" class="form-control" placeholder="Rancher密码" />
            </div>
          </div>
          
          <!-- Kubernetes配置 -->
          <div id="kubernetesConfig" class="resource-config" style="display: none;">
            <div class="form-group">
              <label>Kubeconfig文件路径</label>
              <div style="display: flex; gap: 10px;">
                <input type="text" id="kubeconfigPath" class="form-control" placeholder="~/.kube/config" />
                <button class="btn-secondary" onclick="selectKubeconfigFile()">选择文件</button>
              </div>
            </div>
            <div class="form-group">
              <label>Context（可选）</label>
              <input type="text" id="kubeContext" class="form-control" placeholder="production" />
            </div>
            <div class="form-group">
              <label>Namespace（可选）</label>
              <input type="text" id="kubeNamespace" class="form-control" placeholder="default" />
            </div>
          </div>
          
          <div class="modal-actions">
            <button class="btn-cancel" onclick="closeAddResourceModal()">取消</button>
            <button class="btn-test" onclick="testResourceConnection()">测试连接</button>
            <button class="btn-primary" onclick="saveResource()">保存</button>
          </div>
        </div>
      </div>
    </div>
  `
}

// 渲染资源列表
function renderResources() {
  if (resources.length === 0) {
    return `
      <div class="empty-state">
        <p>暂无资源</p>
        <p>点击"+ 添加"按钮添加资源</p>
      </div>
    `
  }
  
  return resources.map(resource => `
    <div class="resource-item ${selectedResourceIds.includes(resource.id) ? 'selected' : ''}" 
         onclick="toggleResourceSelection('${resource.id}')">
      <input type="checkbox" class="resource-checkbox" 
             ${selectedResourceIds.includes(resource.id) ? 'checked' : ''}
             onclick="event.stopPropagation(); toggleResourceSelection('${resource.id}')" />
      <div class="resource-icon">${getResourceIcon(resource.type)}</div>
      <div class="resource-info">
        <div class="resource-name">${resource.name}</div>
        <div class="resource-details">${getResourceDetails(resource)}</div>
      </div>
    </div>
  `).join('')
}

function getResourceIcon(type) {
  const icons = {
    ssh: '🖥️',
    rancher: '🐄',
    kubernetes: '☸️'
  }
  return icons[type] || '📦'
}

function getResourceDetails(resource) {
  if (resource.type === 'ssh') {
    return `${resource.host}:${resource.port || 22}`
  } else if (resource.type === 'rancher') {
    return resource.url || ''
  } else if (resource.type === 'kubernetes') {
    return resource.context || resource.kubeconfigPath || ''
  }
  return ''
}

// 设置事件监听器
function setupEventListeners() {
  // 添加资源按钮
  document.getElementById('addResourceBtn')?.addEventListener('click', showAddResourceModal)
  
  // 资源类型切换
  document.getElementById('resourceTypeSelect')?.addEventListener('change', onResourceTypeChange)
  
  // SSH用户名输入监听（判断是否是root）
  document.getElementById('sshUsername')?.addEventListener('input', onSSHUsernameChange)
  
  // 提权方式切换
  document.getElementById('privilegeMethod')?.addEventListener('change', onPrivilegeMethodChange)
  
  // 发送消息按钮
  document.getElementById('sendBtn')?.addEventListener('click', sendMessage)
  
  // 回车发送
  document.getElementById('chatInput')?.addEventListener('keydown', (e) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      sendMessage()
    }
  })
  
  // 点击模态框外部关闭
  document.getElementById('addResourceModal')?.addEventListener('click', (e) => {
    if (e.target.id === 'addResourceModal') {
      closeAddResourceModal()
    }
  })
}

// 显示添加资源对话框
function showAddResourceModal() {
  document.getElementById('addResourceModal').style.display = 'flex'
  currentResourceType = 'ssh'
  onResourceTypeChange()
}

// 关闭添加资源对话框
function closeAddResourceModal() {
  document.getElementById('addResourceModal').style.display = 'none'
  // 清空表单
  document.querySelectorAll('input.form-control').forEach(input => {
    input.value = ''
  })
  document.querySelectorAll('select.form-control').forEach(select => {
    if (select.id === 'resourceTypeSelect') {
      select.value = 'ssh'
    } else if (select.id === 'privilegeMethod') {
      select.value = 'sudo'
    }
  })
  document.getElementById('sshPort').value = '22'
  // 重置提权配置显示
  const privilegeConfig = document.getElementById('privilegeConfig')
  const suPasswordGroup = document.getElementById('suPasswordGroup')
  if (privilegeConfig) privilegeConfig.style.display = 'none'
  if (suPasswordGroup) suPasswordGroup.style.display = 'none'
}

// 资源类型切换
function onResourceTypeChange() {
  currentResourceType = document.getElementById('resourceTypeSelect').value
  
  // 隐藏所有配置
  document.getElementById('sshConfig').style.display = 'none'
  document.getElementById('rancherConfig').style.display = 'none'
  document.getElementById('kubernetesConfig').style.display = 'none'
  
  // 显示选中的配置
  if (currentResourceType === 'ssh') {
    document.getElementById('sshConfig').style.display = 'block'
    // 检查用户名并更新提权配置显示
    onSSHUsernameChange()
  } else if (currentResourceType === 'rancher') {
    document.getElementById('rancherConfig').style.display = 'block'
  } else if (currentResourceType === 'kubernetes') {
    document.getElementById('kubernetesConfig').style.display = 'block'
  }
}

// SSH用户名变化处理
function onSSHUsernameChange() {
  const username = document.getElementById('sshUsername')?.value.trim().toLowerCase()
  const privilegeConfig = document.getElementById('privilegeConfig')
  
  if (!privilegeConfig) return
  
  // 如果是root用户，隐藏提权配置
  if (username === 'root') {
    privilegeConfig.style.display = 'none'
  } else if (username) {
    // 非root用户，显示提权配置
    privilegeConfig.style.display = 'block'
    // 检查提权方式，如果是su则显示root密码输入框
    onPrivilegeMethodChange()
  } else {
    // 用户名为空，隐藏提权配置
    privilegeConfig.style.display = 'none'
  }
}

// 提权方式切换处理
function onPrivilegeMethodChange() {
  const method = document.getElementById('privilegeMethod')?.value
  const suPasswordGroup = document.getElementById('suPasswordGroup')
  
  if (!suPasswordGroup) return
  
  // 如果选择su，显示root密码输入框
  if (method === 'su') {
    suPasswordGroup.style.display = 'block'
  } else {
    suPasswordGroup.style.display = 'none'
  }
}

// 测试连接
async function testResourceConnection() {
  const resourceType = document.getElementById('resourceTypeSelect').value
  
  if (resourceType === 'ssh') {
    const host = document.getElementById('sshHost').value
    const port = document.getElementById('sshPort').value
    const username = document.getElementById('sshUsername').value.trim()
    const password = document.getElementById('sshPassword').value
    
    if (!host || !username || !password) {
      alert('请填写IP地址、用户名和密码')
      return
    }
    
    // 禁用测试按钮，显示加载状态
    const testBtn = document.querySelector('.btn-test')
    const originalText = testBtn.textContent
    testBtn.disabled = true
    testBtn.textContent = '测试中...'
    
    try {
      const result = await invoke('test_ssh_connection', {
        host: host,
        port: parseInt(port) || 22,
        username: username,
        password: password
      })
      
      alert('✓ ' + result)
    } catch (error) {
      alert('✗ 连接测试失败: ' + error)
    } finally {
      testBtn.disabled = false
      testBtn.textContent = originalText
    }
  } else {
    alert('当前资源类型的测试连接功能待实现')
  }
}

// 选择kubeconfig文件
async function selectKubeconfigFile() {
  // TODO: 使用Tauri的文件选择对话框
  alert('文件选择功能待实现')
}

// 保存资源
async function saveResource() {
  const resourceType = document.getElementById('resourceTypeSelect').value
  let resource = {
    id: Date.now().toString(),
    type: resourceType,
    name: '',
  }
  
  if (resourceType === 'ssh') {
    const host = document.getElementById('sshHost').value
    const port = document.getElementById('sshPort').value
    const username = document.getElementById('sshUsername').value.trim()
    const password = document.getElementById('sshPassword').value
    
    if (!host || !username) {
      alert('请填写IP地址和用户名')
      return
    }
    
    resource.name = `${username}@${host}`
    resource.host = host
    resource.port = parseInt(port) || 22
    resource.username = username
    resource.password = password
    resource.useRoot = username.toLowerCase() === 'root'
    
    // 如果不是root用户，保存提权配置
    if (!resource.useRoot) {
      const privilegeMethod = document.getElementById('privilegeMethod').value
      resource.privilegeMethod = privilegeMethod
      
      // 如果使用su，需要root密码
      if (privilegeMethod === 'su') {
        const suPassword = document.getElementById('suPassword').value
        if (!suPassword) {
          alert('使用su提权方式需要提供Root密码')
          return
        }
        resource.suPassword = suPassword
      }
    }
  } else if (resourceType === 'rancher') {
    const url = document.getElementById('rancherUrl').value
    const username = document.getElementById('rancherUsername').value
    
    if (!url || !username) {
      alert('请填写Rancher地址和用户名')
      return
    }
    
    resource.name = `Rancher-${new URL(url).hostname}`
    resource.url = url
    resource.username = username
    resource.password = document.getElementById('rancherPassword').value
  } else if (resourceType === 'kubernetes') {
    const kubeconfigPath = document.getElementById('kubeconfigPath').value
    const context = document.getElementById('kubeContext').value
    
    if (!kubeconfigPath) {
      alert('请填写Kubeconfig文件路径')
      return
    }
    
    resource.name = context || `K8s-${kubeconfigPath.split('/').pop()}`
    resource.kubeconfigPath = kubeconfigPath
    resource.context = context
    resource.namespace = document.getElementById('kubeNamespace').value
  }
  
  // 添加到资源列表
  resources.push(resource)
  
  // 重新渲染
  renderApp()
  setupEventListeners()
  
  // 关闭对话框
  closeAddResourceModal()
  
  alert('资源添加成功！')
}

// 切换资源选择
function toggleResourceSelection(resourceId) {
  const index = selectedResourceIds.indexOf(resourceId)
  if (index > -1) {
    selectedResourceIds.splice(index, 1)
  } else {
    selectedResourceIds.push(resourceId)
  }
  
  // 更新UI
  updateSelectedInfo()
  updateChatInputState()
}

// 更新选中信息
function updateSelectedInfo() {
  const selectedInfo = document.getElementById('selectedInfo')
  if (selectedResourceIds.length === 0) {
    selectedInfo.innerHTML = '<span>请选择资源</span>'
  } else if (selectedResourceIds.length === 1) {
    const resource = resources.find(r => r.id === selectedResourceIds[0])
    selectedInfo.innerHTML = `<span>已选择: ${resource?.name || ''}</span>`
  } else {
    selectedInfo.innerHTML = `<span>已选择 ${selectedResourceIds.length} 个资源</span>`
  }
}

// 更新聊天输入状态
function updateChatInputState() {
  const chatInput = document.getElementById('chatInput')
  const sendBtn = document.getElementById('sendBtn')
  const enabled = selectedResourceIds.length > 0
  
  chatInput.disabled = !enabled
  sendBtn.disabled = !enabled
}

// 发送消息
async function sendMessage() {
  const input = document.getElementById('chatInput')
  const message = input.value.trim()
  
  if (!message || selectedResourceIds.length === 0) {
    return
  }
  
  // 添加用户消息
  addMessage('user', message)
  input.value = ''
  
  // 构建上下文
  const selectedResources = resources.filter(r => selectedResourceIds.includes(r.id))
  const context = `已选择资源：${selectedResources.map(r => r.name).join(', ')}`
  
  try {
    // 调用AI
    const response = await invoke('chat_with_ai', {
      message: message,
      context: context
    })
    
    addMessage('ai', response)
  } catch (error) {
    addMessage('ai', `错误: ${error}`)
  }
}

// 添加消息
function addMessage(role, content) {
  const container = document.getElementById('messagesContainer')
  const messageDiv = document.createElement('div')
  messageDiv.className = `message-item ${role}`
  messageDiv.innerHTML = `
    <div class="message-header">
      <span class="message-role">${role === 'user' ? '你' : 'AI'}</span>
      <span class="message-time">${new Date().toLocaleTimeString()}</span>
    </div>
    <div class="message-content">${content}</div>
  `
  container.appendChild(messageDiv)
  container.scrollTop = container.scrollHeight
}

// 检查LLM健康状态
async function checkLLMHealth() {
  try {
    const isHealthy = await invoke('check_llm_health')
    const statusEl = document.getElementById('llmStatus')
    if (isHealthy) {
      statusEl.textContent = '● AI已就绪'
      statusEl.className = 'status-indicator online'
    } else {
      statusEl.textContent = '● AI未加载'
      statusEl.className = 'status-indicator offline'
    }
  } catch (error) {
    console.error('LLM健康检查失败:', error)
  }
}

// 全局函数（供HTML调用）
window.showAddResourceModal = showAddResourceModal
window.closeAddResourceModal = closeAddResourceModal
window.onResourceTypeChange = onResourceTypeChange
window.onSSHUsernameChange = onSSHUsernameChange
window.onPrivilegeMethodChange = onPrivilegeMethodChange
window.testResourceConnection = testResourceConnection
window.selectKubeconfigFile = selectKubeconfigFile
window.saveResource = saveResource
window.toggleResourceSelection = toggleResourceSelection

// 启动应用
initApp()
