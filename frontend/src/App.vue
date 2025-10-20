<template>
  <div class="app">
    <!-- 错误提示模态框 -->
    <div v-if="showError" class="error-modal">
      <div class="error-modal-content">
        <h3 class="error-title">{{ errorTitle }}</h3>
        <p class="error-message">{{ errorMessage }}</p>
        <button class="error-close-btn" @click="closeError">确定</button>
      </div>
    </div>
    
    <!-- 模型下载页面 -->
    <div v-if="showModelDownload" class="model-download-page">
      <div class="download-container">
        <h1>Lohan AI系统分析师</h1>
        <p class="subtitle">请下载一个AI模型以继续使用</p>
        
        <div class="model-list">
          <div 
            v-for="model in availableModels" 
            :key="model.name" 
            class="model-card"
            :class="{ 'downloading': downloadingModel === model.name }"
          >
            <h3>{{ model.name }}</h3>
            <div class="model-info">
              <span class="model-size">{{ model.size }}</span>
              <p class="model-description">{{ model.description }}</p>
            </div>
            <button 
              @click="downloadSelectedModel(model.name)"
              :disabled="downloadingModel !== null"
              class="btn-download"
            >
              {{ downloadingModel === model.name ? '下载中...' : '下载' }}
            </button>
          </div>
        </div>
        
        <div v-if="downloadingModel" class="download-progress">
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: downloadProgress + '%' }"></div>
          </div>
          <span class="progress-text">{{ downloadProgress }}%</span>
        </div>
      </div>
    </div>
    
    <!-- 主应用页面 -->
    <div v-else>
      <!-- 顶部工具栏 -->
      <header class="header">
        <h1>Lohan</h1>
        <div class="status">
          <span v-if="modelLoaded" class="status-indicator online">● AI已就绪</span>
          <span v-else class="status-indicator offline">● AI未加载</span>
        </div>
      </header>

      <div class="main-container">
      <!-- 左侧服务器列表 -->
      <aside class="server-list">
        <div class="server-list-header">
          <h2>服务器列表</h2>
          <div class="header-buttons">
            <button @click="refreshConnections" class="btn-refresh" title="刷新连接">⟳</button>
            <button @click="showAddServerDialog" class="btn-add">+ 添加</button>
          </div>
        </div>

        <div class="servers">
          <div 
            v-for="server in servers" 
            :key="server.id"
            class="server-item"
            :class="{ 
              selected: selectedServerIds.includes(server.id),
              'connected': serverConnectionStatus[server.id],
              'disconnected': serverConnectionStatus[server.id] === false
            }"
            @click="toggleServerSelection(server.id)"
            @contextmenu.prevent="showContextMenu($event, server)"
          >
            <div class="server-info">
              <div class="server-name">{{ server.name }}</div>
              <div class="server-ip">{{ server.ip }}:{{ server.ssh_port }}</div>
            </div>
          </div>

          <div v-if="servers.length === 0" class="empty-state">
            <p>暂无服务器</p>
            <p>点击"+ 添加"按钮添加服务器</p>
          </div>
        </div>
        
        <!-- 右键菜单 -->
        <div 
          v-if="contextMenu.visible" 
          class="context-menu"
          :style="{ left: contextMenu.x + 'px', top: contextMenu.y + 'px' }"
          @click.stop
        >
          <div class="menu-item" @click="handleEditServer">编辑</div>
          <div class="menu-item" @click="handleDeleteServer">删除</div>
        </div>
      </aside>

      <!-- 右侧聊天区域 -->
      <main class="chat-area">
        <div class="chat-header">
          <div class="selected-info">
            <span v-if="selectedServerIds.length === 0">请选择服务器</span>
            <span v-else-if="selectedServerIds.length === 1">已选择: {{ getServerName(selectedServerIds[0]) }}</span>
            <span v-else>已选择 {{ selectedServerIds.length }} 台服务器</span>
          </div>
        </div>

        <div class="chat-messages" ref="messagesContainer">
          <div v-for="(message, index) in messages" :key="index" :class="['message', message.role]">
            <div class="message-content">
              <div class="message-text">{{ message.content }}</div>
              <div class="message-time">{{ message.time }}</div>
            </div>
          </div>
        </div>

        <div class="chat-input">
          <textarea 
            v-model="inputMessage"
            @keydown.enter.exact.prevent="sendMessage"
            placeholder="输入消息... (按Enter发送，Shift+Enter换行)"
            rows="3"
          ></textarea>
          <button @click="sendMessage" :disabled="!inputMessage.trim() || selectedServerIds.length === 0" class="btn-send">
            发送
          </button>
        </div>
      </main>
    </div>

    <!-- 添加/编辑服务器对话框 -->
    <div v-if="showServerDialog" class="modal-overlay" @click="closeServerDialog">
      <div class="modal-content" @click.stop>
        <h2>{{ editingServer ? '编辑服务器' : '添加服务器' }}</h2>

        <div class="form-group">
          <label>IP地址</label>
          <input v-model="serverForm.ip" type="text" placeholder="例如: 192.168.1.100" />
        </div>

        <div class="form-group">
          <label>SSH端口</label>
          <input v-model.number="serverForm.ssh_port" type="number" placeholder="默认: 22" />
        </div>

        <div class="form-group">
          <label>用户名</label>
          <input v-model="serverForm.username" type="text" placeholder="例如: root / ubuntu" @input="checkIsRoot" />
        </div>

        <div class="form-group">
          <label>密码</label>
          <input v-model="serverForm.password" type="password" placeholder="SSH密码" />
        </div>

        <div v-if="!isRootUser" class="form-group">
          <label>提权方式</label>
          <select v-model="serverForm.sudo_method">
            <option value="sudo">sudo</option>
            <option value="su">su</option>
          </select>
        </div>

        <div v-if="!isRootUser" class="form-group">
          <label>提权密码</label>
          <input v-model="serverForm.sudo_pass" type="password" placeholder="sudo/su密码" />
        </div>

        <div class="modal-actions">
          <button @click="closeServerDialog" class="btn-cancel">取消</button>
          <button @click="testConnection" class="btn-test" :disabled="!canTestConnection">测试连接</button>
          <button @click="saveServer" class="btn-primary">保存</button>
        </div>
      </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted, nextTick, computed } from 'vue'
import { GetServers, AddServer, UpdateServer, DeleteServer, IsModelLoaded, Ask, TestConnection, GetServerConnectionStatus, RefreshServerConnections, CheckModelExists, GetAvailableModels, DownloadModel } from '../wailsjs/go/main/App'

export default {
  name: 'App',
  setup() {
    const servers = ref([])
    const selectedServerIds = ref([])
    const messages = ref([])
    const inputMessage = ref('')
    const modelLoaded = ref(false)
    const modelExists = ref(false)
    const showModelDownload = ref(false)
    const availableModels = ref([])
    const downloadingModel = ref(null)
    const downloadProgress = ref(0)
    const messagesContainer = ref(null)
    const serverConnectionStatus = ref({}) // 存储服务器连接状态
    // 错误提示相关状态
    const showError = ref(false)
    const errorTitle = ref('')
    const errorMessage = ref('')

    // 服务器对话框
    const showServerDialog = ref(false)
    const editingServer = ref(null)
    const isRootUser = ref(false)
    const serverForm = ref({
      id: '',
      name: '',
      ip: '',
      ssh_port: 22,
      username: '',
      password: '',
      use_root: false,
      sudo_method: 'sudo',
      sudo_pass: ''
    })
    
    // 右键菜单状态
    const contextMenu = ref({
      visible: false,
      x: 0,
      y: 0,
      server: null
    })

    // 检查是否是root用户
    const checkIsRoot = () => {
      isRootUser.value = serverForm.value.username.toLowerCase() === 'root'
      serverForm.value.use_root = isRootUser.value
    }

    // 检查是否可以测试连接
    const canTestConnection = computed(() => {
      return serverForm.value.ip && 
             serverForm.value.username && 
             serverForm.value.password &&
             serverForm.value.ssh_port > 0
    })

    // 加载服务器列表
    const loadServers = async () => {
      try {
        const result = await GetServers()
        servers.value = result || []
        // 加载后立即获取连接状态
        await loadServerConnectionStatus()
      } catch (error) {
        console.error('Failed to load servers:', error)
        servers.value = []
      }
    }

    // 加载服务器连接状态
    const loadServerConnectionStatus = async () => {
      try {
        const status = await GetServerConnectionStatus()
        serverConnectionStatus.value = status
      } catch (error) {
        console.error('Failed to load server connection status:', error)
      }
    }

    // 刷新所有服务器连接
    const refreshConnections = async () => {
      try {
        await RefreshServerConnections()
        // 延迟一下再获取状态，让后端有时间建立连接
        setTimeout(loadServerConnectionStatus, 1000)
      } catch (error) {
        console.error('Failed to refresh connections:', error)
      }
    }

    // 检查模型是否加载
    const checkModelLoaded = async () => {
      try {
        modelLoaded.value = await IsModelLoaded()
      } catch (error) {
        console.error('Failed to check model status:', error)
      }
    }
    
    // 检查模型文件是否存在
    const checkModelExists = async () => {
      try {
        modelExists.value = await CheckModelExists()
        if (!modelExists.value) {
          // 显示模型下载页面
          showModelDownload.value = true
          await loadAvailableModels()
        }
      } catch (error) {
        console.error('Failed to check model existence:', error)
        // 如果检查失败，也显示下载页面
        showModelDownload.value = true
        await loadAvailableModels()
      }
    }
    
    // 加载可用模型列表
    const loadAvailableModels = async () => {
      try {
        availableModels.value = await GetAvailableModels()
      } catch (error) {
        console.error('Failed to load available models:', error)
      }
    }
    
    // 显示错误信息
    const showErrorMessage = (title, message) => {
      console.log('显示错误:', title, message)
      
      // 1. 使用标准alert作为最可靠的备选方案
      alert(`${title}\n${message}`)
      
      // 2. 同时更新Vue组件中的错误提示
      errorTitle.value = title
      errorMessage.value = message
      showError.value = true
      
      // 8秒后自动隐藏
      setTimeout(() => {
        showError.value = false
      }, 8000)
    }
    
    // 关闭错误信息
    const closeError = () => {
      showError.value = false
    }
    
    // 下载模型
    const downloadSelectedModel = async (modelName) => {
      try {
        downloadingModel.value = modelName
        downloadProgress.value = 0
        
        console.log('开始下载模型:', modelName)
        // 这里需要在Go端修改DownloadModel方法，使其能够返回进度
        // 暂时直接调用下载
        await DownloadModel(modelName)
        
        // 下载完成后，重新检查模型是否存在
        modelExists.value = await CheckModelExists()
        if (modelExists.value) {
          showModelDownload.value = false
          // 尝试加载模型
          await checkModelLoaded()
        }
      } catch (error) {
        console.error('Failed to download model:', error)
        console.error('Error type:', typeof error)
        
        // 确保能够正确获取错误信息
        let errMsg = '下载失败！'
        
        // 尝试多种方式提取错误信息
        if (error) {
          if (typeof error === 'string') {
            errMsg = '下载失败：' + error
          } else if (error.message) {
            errMsg = '下载失败：' + error.message
          } else if (error.error) {
            errMsg = '下载失败：' + error.error
          } else {
            try {
              errMsg = '下载失败：' + JSON.stringify(error)
            } catch (e) {
              errMsg = '下载失败：无法解析错误信息'
            }
          }
        }
        
        console.log('显示的错误信息:', errMsg)
        // 使用Vue响应式数据显示错误
        showErrorMessage('下载错误', errMsg)
      } finally {
        downloadingModel.value = null
        downloadProgress.value = 0
        console.log('下载操作完成')
      }
    }

    // 切换服务器选择
    const toggleServerSelection = (serverId) => {
      const index = selectedServerIds.value.indexOf(serverId)
      if (index > -1) {
        selectedServerIds.value.splice(index, 1)
      } else {
        selectedServerIds.value.push(serverId)
      }
    }

    // 获取服务器名称
    const getServerName = (serverId) => {
      const server = servers.value.find(s => s.id === serverId)
      return server ? server.name : ''
    }

    // 显示添加服务器对话框
    const showAddServerDialog = () => {
      editingServer.value = null
      isRootUser.value = false
      serverForm.value = {
        id: '',
        name: '',
        ip: '',
        ssh_port: 22,
        username: '',
        password: '',
        use_root: false,
        sudo_method: 'sudo',
        sudo_pass: ''
      }
      showServerDialog.value = true
    }

    // 编辑服务器
    const editServer = (server) => {
      console.log('Editing server:', server)
      editingServer.value = server
      serverForm.value = { ...server }
      isRootUser.value = server.username.toLowerCase() === 'root'
      showServerDialog.value = true
    }
    
    // 右键菜单相关函数
    const showContextMenu = (event, server) => {
      contextMenu.value = {
        visible: true,
        x: event.clientX,
        y: event.clientY,
        server: server
      }
    }
    
    const hideContextMenu = () => {
      contextMenu.value.visible = false
    }
    
    const handleEditServer = () => {
      if (contextMenu.value.server) {
        editServer(contextMenu.value.server)
        hideContextMenu()
      }
    }
    
    const handleDeleteServer = () => {
      if (contextMenu.value.server) {
        deleteServer(contextMenu.value.server.id)
        hideContextMenu()
      }
    }

    // 关闭对话框
    const closeServerDialog = () => {
      showServerDialog.value = false
      editingServer.value = null
    }

    // 测试连接
    const testConnection = async () => {
      try {
        await TestConnection(serverForm.value)
        alert('✓ 连接测试成功！')
      } catch (error) {
        console.error('Connection test failed:', error)
        alert('✗ 连接测试失败: ' + error)
      }
    }

    // 保存服务器
    const saveServer = async () => {
      try {
        // 自动生成服务器名称：username@ip
        serverForm.value.name = `${serverForm.value.username}@${serverForm.value.ip}`
        console.log('Saving server:', serverForm.value)
        
        if (editingServer.value) {
          console.log('Updating existing server')
          await UpdateServer(serverForm.value)
        } else {
          console.log('Adding new server')
          await AddServer(serverForm.value)
        }
        await loadServers()
        closeServerDialog()
      } catch (error) {
        console.error('Failed to save server:', error)
        alert('保存失败: ' + error)
      }
    }

    // 删除服务器
    const deleteServer = async (serverId) => {
      console.log('删除按钮被点击，服务器ID:', serverId)
      
      // 直接执行删除操作，不使用对话框
      try {
        console.log('开始删除服务器...')
        await DeleteServer(serverId)
        console.log('服务器删除成功，重新加载列表...')
        await loadServers()
        // 从选中列表中移除
        const index = selectedServerIds.value.indexOf(serverId)
        if (index > -1) {
          selectedServerIds.value.splice(index, 1)
        }
        console.log('删除完成')
      } catch (error) {
        console.error('删除失败:', error)
      }
    }

    // 发送消息
    const sendMessage = async () => {
      if (!inputMessage.value.trim() || selectedServerIds.value.length === 0) {
        return
      }

      const userMessage = inputMessage.value.trim()
      const timestamp = new Date().toLocaleTimeString()

      // 添加用户消息
      messages.value.push({
        role: 'user',
        content: userMessage,
        time: timestamp
      })

      inputMessage.value = ''

      // 滚动到底部
      await nextTick()
      if (messagesContainer.value) {
        messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
      }

      // 发送到AI
      try {
        // 构建上下文信息
        const selectedServers = servers.value.filter(s => selectedServerIds.value.includes(s.id))
        const serverInfo = selectedServers.map(s => `${s.name} (${s.ip})`).join(', ')
        
        const contextMessage = `用户选择了以下服务器: ${serverInfo}\n\n用户问题: ${userMessage}`
        
        const response = await Ask(contextMessage)
        
        // 添加AI响应
        messages.value.push({
          role: 'assistant',
          content: response,
          time: new Date().toLocaleTimeString()
        })

        // 滚动到底部
        await nextTick()
        if (messagesContainer.value) {
          messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
        }
      } catch (error) {
        console.error('Failed to send message:', error)
        messages.value.push({
          role: 'system',
          content: '发送失败: ' + error,
          time: new Date().toLocaleTimeString()
        })
      }
    }

    onMounted(() => {
      // 首先检查模型是否存在
      checkModelExists()
      // 只有在模型存在时才加载服务器列表
      if (modelExists.value) {
        loadServers()
      }
      checkModelLoaded()
      
      // 定时刷新连接状态（每30秒）
      const interval = setInterval(loadServerConnectionStatus, 30000)
      
      // 点击页面其他地方关闭右键菜单
      const handleClickOutside = () => {
        hideContextMenu()
      }
      
      document.addEventListener('click', handleClickOutside)
      
      // 组件卸载时清除定时器和事件监听器
      return () => {
        clearInterval(interval)
        document.removeEventListener('click', handleClickOutside)
      }
    })

    return {
      servers,
      selectedServerIds,
      messages,
      inputMessage,
      modelLoaded,
      modelExists,
      showModelDownload,
      availableModels,
      downloadingModel,
      downloadProgress,
      showError,
      errorTitle,
      errorMessage,
      messagesContainer,
      showServerDialog,
      editingServer,
      serverForm,
      contextMenu,
      isRootUser,
      canTestConnection,
      serverConnectionStatus,
      toggleServerSelection,
      getServerName,
      showAddServerDialog,
      editServer,
      closeServerDialog,
      testConnection,
      saveServer,
      deleteServer,
      sendMessage,
      checkIsRoot,
      refreshConnections,
      showContextMenu,
      hideContextMenu,
      handleEditServer,
      handleDeleteServer,
      downloadSelectedModel,
      closeError
    }
  }
}
</script>

<style scoped>
* {
  box-sizing: border-box;
}

.app {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: #1e1e1e;
  color: #e0e0e0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

/* 错误提示模态框样式 */
.error-modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 99999;
}

.error-modal-content {
  background: #ff4757;
  color: white;
  padding: 30px;
  border-radius: 12px;
  max-width: 500px;
  width: 90%;
  text-align: center;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
}

.error-title {
  margin-top: 0;
  margin-bottom: 15px;
  font-size: 24px;
  font-weight: bold;
}

.error-message {
  margin-bottom: 20px;
  font-size: 16px;
  line-height: 1.5;
  word-break: break-word;
}

.error-close-btn {
  background: white;
  color: #ff4757;
  border: none;
  padding: 12px 30px;
  border-radius: 6px;
  font-size: 16px;
  font-weight: bold;
  cursor: pointer;
  transition: background-color 0.2s;
}

.error-close-btn:hover {
  background: #f0f0f0;
}

/* 模型下载页面样式 */
.model-download-page {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #1e1e1e;
  padding: 20px;
}

.download-container {
  width: 100%;
  max-width: 800px;
  text-align: center;
}

.download-container h1 {
  font-size: 32px;
  margin-bottom: 8px;
  color: #ffffff;
}

.subtitle {
  font-size: 18px;
  color: #999;
  margin-bottom: 40px;
}

.model-list {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
  margin-bottom: 30px;
}

.model-card {
  background: #252525;
  border-radius: 8px;
  padding: 20px;
  text-align: left;
  transition: transform 0.2s, box-shadow 0.2s;
  border: 1px solid #3a3a3a;
}

.model-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.model-card.downloading {
  opacity: 0.8;
}

.model-card h3 {
  margin-top: 0;
  margin-bottom: 12px;
  color: #ffffff;
  font-size: 18px;
}

.model-info {
  margin-bottom: 16px;
}

.model-size {
  display: inline-block;
  background: #007acc;
  color: white;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 12px;
  margin-bottom: 8px;
}

.model-description {
  font-size: 14px;
  color: #ccc;
  line-height: 1.5;
  margin: 0;
}

.btn-download {
  width: 100%;
  padding: 10px;
  background: #007acc;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  font-weight: 500;
  transition: background-color 0.2s;
}

.btn-download:hover:not(:disabled) {
  background: #005a9e;
}

.btn-download:disabled {
  background: #555;
  cursor: not-allowed;
}

.download-progress {
  margin-top: 20px;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: #3a3a3a;
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 8px;
}

.progress-fill {
  height: 100%;
  background: #007acc;
  transition: width 0.3s;
}

.progress-text {
  color: #999;
  font-size: 14px;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 20px;
  background: #252525;
  border-bottom: 1px solid #3a3a3a;
}

.header h1 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #fff;
}

.status {
  display: flex;
  align-items: center;
}

.status-indicator {
  font-size: 14px;
  padding: 4px 12px;
  border-radius: 12px;
}

.status-indicator.online {
  color: #4caf50;
  background: rgba(76, 175, 80, 0.1);
}

.status-indicator.offline {
  color: #999;
  background: rgba(153, 153, 153, 0.1);
}

.main-container {
  display: flex;
  flex: 1;
  overflow: hidden;
}

/* 服务器列表样式 */
.server-list {
  width: 280px;
  background: #252525;
  border-right: 1px solid #3a3a3a;
  display: flex;
  flex-direction: column;
}

.server-list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid #3a3a3a;
}

.server-list-header h2 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.btn-add {
  padding: 6px 12px;
  background: #007acc;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.btn-add:hover {
  background: #005a9e;
}

.servers {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.server-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  margin-bottom: 8px;
  background: #2d2d2d;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.server-item:hover {
  background: #333;
}

.server-item.selected {
  background: #007acc;
}

/* 连接状态样式 */
.server-item {
  background-color: #2d2d2d;
  border-left: 5px solid #555555;
  border-radius: 4px;
  border: none;
  color: #cccccc;
}

.server-item.connected {
  border-left-color: #28a745;
}

.server-item.disconnected {
  border-left-color: #dc3545;
}

.server-item.selected {
  background-color: #3a3a3a;
  border-left-width: 5px;
  border-right: 1px solid #555555;
  border-top: 1px solid #555555;
  border-bottom: 1px solid #555555;
  color: #ffffff;
}

.server-item:hover {
  background-color: #353535;
  transition: background-color 0.2s ease;
}

.server-item.selected:hover {
  background-color: #424242;
}

.server-info {
  flex: 1;
}

.server-name {
  font-weight: 600;
  margin-bottom: 4px;
}

.server-ip {
  font-size: 12px;
  color: #999;
}

.server-item.selected .server-ip {
  color: #e0e0e0;
}

/* 右键菜单样式 */
.context-menu {
  position: fixed;
  background: #333;
  border: 1px solid #555;
  border-radius: 4px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  z-index: 1000;
  min-width: 120px;
}

.menu-item {
  padding: 10px 16px;
  color: #e0e0e0;
  cursor: pointer;
  transition: background-color 0.2s;
}

.menu-item:hover {
  background: #007acc;
  color: white;
}

.menu-item:first-child {
  border-top-left-radius: 4px;
  border-top-right-radius: 4px;
}

.menu-item:last-child {
  border-bottom-left-radius: 4px;
  border-bottom-right-radius: 4px;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: #666;
}

.empty-state p {
  margin: 8px 0;
}

/* 聊天区域样式 */
.chat-area {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.chat-header {
  padding: 16px 20px;
  background: #252525;
  border-bottom: 1px solid #3a3a3a;
}

.selected-info {
  font-size: 14px;
  color: #999;
}

.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.message {
  margin-bottom: 16px;
  display: flex;
}

.message.user {
  justify-content: flex-end;
}

.message-content {
  max-width: 70%;
  padding: 12px 16px;
  border-radius: 8px;
}

.message.user .message-content {
  background: #007acc;
  color: white;
}

.message.assistant .message-content {
  background: #2d2d2d;
}

.message.system .message-content {
  background: #d32f2f;
  color: white;
}

.message-text {
  white-space: pre-wrap;
  word-wrap: break-word;
}

.message-time {
  font-size: 11px;
  margin-top: 4px;
  opacity: 0.6;
}

.chat-input {
  display: flex;
  gap: 12px;
  padding: 16px 20px;
  background: #252525;
  border-top: 1px solid #3a3a3a;
}

.chat-input textarea {
  flex: 1;
  padding: 12px;
  background: #2d2d2d;
  border: 1px solid #3a3a3a;
  border-radius: 6px;
  color: #e0e0e0;
  font-family: inherit;
  font-size: 14px;
  resize: none;
}

.chat-input textarea:focus {
  outline: none;
  border-color: #007acc;
}

.btn-send {
  padding: 12px 24px;
  background: #007acc;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
}

.btn-send:hover:not(:disabled) {
  background: #005a9e;
}

.btn-send:disabled {
  background: #555;
  cursor: not-allowed;
}

/* 模态框样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: #2d2d2d;
  border-radius: 8px;
  padding: 24px;
  width: 90%;
  max-width: 500px;
  max-height: 80vh;
  overflow-y: auto;
}

.modal-content h2 {
  margin: 0 0 20px 0;
  font-size: 20px;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 6px;
  font-size: 14px;
  color: #ccc;
}

.form-group input[type="text"],
.form-group input[type="password"],
.form-group input[type="number"],
.form-group select {
  width: 100%;
  padding: 10px;
  background: #1e1e1e;
  border: 1px solid #3a3a3a;
  border-radius: 4px;
  color: #e0e0e0;
  font-size: 14px;
}

.form-group input[type="text"]:focus,
.form-group input[type="password"]:focus,
.form-group input[type="number"]:focus,
.form-group select:focus {
  outline: none;
  border-color: #007acc;
}

.form-group input[type="checkbox"] {
  margin-right: 8px;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
}

.btn-cancel {
  padding: 10px 20px;
  background: transparent;
  border: 1px solid #3a3a3a;
  border-radius: 4px;
  color: #e0e0e0;
  cursor: pointer;
}

.btn-cancel:hover {
  background: #3a3a3a;
}

.btn-primary {
  padding: 10px 20px;
  background: #007acc;
  border: none;
  border-radius: 4px;
  color: white;
  cursor: pointer;
}

.btn-primary:hover {
  background: #005a9e;
}

.btn-test {
  padding: 10px 20px;
  background: #ff9800;
  border: none;
  border-radius: 4px;
  color: white;
  cursor: pointer;
}

.btn-test:hover:not(:disabled) {
  background: #f57c00;
}

.btn-test:disabled {
  background: #555;
  cursor: not-allowed;
}

/* 滚动条样式 */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: #1e1e1e;
}

::-webkit-scrollbar-thumb {
  background: #555;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #666;
}
</style>
