<template>
  <div class="app">
    <header class="header">
      <h1>Lohan - AI系统分析师</h1>
      <p>AI驱动的系统分析师和运维助手</p>
    </header>
    
    <main class="main">
      <div class="welcome">
        <h2>欢迎使用Lohan</h2>
        <p>自动化系统诊断、核心转储分析、恶意软件清理和数据修复</p>
        
        <div class="demo-section">
          <h3>功能演示</h3>
          <div class="input-group">
            <input 
              v-model="name" 
              type="text" 
              placeholder="请输入您的姓名"
              @keyup.enter="greet"
            />
            <button @click="greet">问候</button>
          </div>
          <div v-if="greeting" class="greeting">
            {{ greeting }}
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script>
import { ref } from 'vue'

export default {
  name: 'App',
  setup() {
    const name = ref('')
    const greeting = ref('')

    const greet = async () => {
      if (name.value.trim()) {
        try {
          // 这里会调用Go后端的Greet方法
          greeting.value = `Hello ${name.value}, It's show time!`
        } catch (error) {
          console.error('Error calling backend:', error)
        }
      }
    }

    return {
      name,
      greeting,
      greet
    }
  }
}
</script>

<style scoped>
.app {
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.header {
  text-align: center;
  padding: 2rem 0;
  background: rgba(0, 0, 0, 0.1);
}

.header h1 {
  margin: 0;
  font-size: 2.5rem;
  font-weight: 300;
}

.header p {
  margin: 0.5rem 0 0 0;
  font-size: 1.1rem;
  opacity: 0.9;
}

.main {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
}

.welcome {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 15px;
  padding: 2rem;
  backdrop-filter: blur(10px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

.welcome h2 {
  margin: 0 0 1rem 0;
  font-size: 2rem;
  font-weight: 400;
}

.welcome p {
  margin: 0 0 2rem 0;
  font-size: 1.1rem;
  opacity: 0.9;
  line-height: 1.6;
}

.demo-section h3 {
  margin: 0 0 1rem 0;
  font-size: 1.5rem;
  font-weight: 400;
}

.input-group {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
}

.input-group input {
  flex: 1;
  padding: 0.75rem 1rem;
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  background: rgba(255, 255, 255, 0.9);
  color: #333;
}

.input-group input:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(255, 255, 255, 0.3);
}

.input-group button {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 8px;
  background: #4CAF50;
  color: white;
  font-size: 1rem;
  cursor: pointer;
  transition: background 0.3s;
}

.input-group button:hover {
  background: #45a049;
}

.greeting {
  padding: 1rem;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  font-size: 1.1rem;
  font-weight: 500;
}
</style>
