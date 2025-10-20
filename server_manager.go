package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"os"
	"sync"
	"time"

	"golang.org/x/crypto/ssh"
)

// ServerInfo represents a server configuration
type ServerInfo struct {
	ID         string       `json:"id"`
	Name       string       `json:"name"`
	IP         string       `json:"ip"`
	SSHPort    int          `json:"ssh_port"`
	Username   string       `json:"username"`
	Password   string       `json:"password,omitempty"`
	PrivateKey string       `json:"private_key,omitempty"`
	UseRoot    bool         `json:"use_root"`
	SudoMethod string       `json:"sudo_method"` // "sudo" or "su"
	SudoPass   string       `json:"sudo_pass,omitempty"`
	// 非持久化字段
	SSHClient   *ssh.Client `json:"-"` // SSH客户端连接对象，不序列化到JSON
	IsConnected bool        `json:"-"` // 连接状态，不序列化到JSON
	LastConnErr error       `json:"-"` // 最后一次连接错误，不序列化到JSON
}

// ServerManager handles server configuration management
type ServerManager struct {
	configFile string
	servers    []ServerInfo
	mu         sync.RWMutex
}

// NewServerManager creates a new server manager instance
func NewServerManager() *ServerManager {
	sm := &ServerManager{
		configFile: "./servers.json",
		servers:    make([]ServerInfo, 0),
	}

	// Load existing configuration
	sm.loadConfig()

	// 尝试连接所有服务器
	sm.connectAllServers()

	return sm
}

// loadConfig loads server configurations from JSON file
func (sm *ServerManager) loadConfig() error {
	sm.mu.Lock()
	defer sm.mu.Unlock()

	file, err := os.Open(sm.configFile)
	if err != nil {
		if os.IsNotExist(err) {
			// Create default empty config
			return sm.saveConfigLocked()
		}
		return err
	}
	defer file.Close()

	data, err := io.ReadAll(file)
	if err != nil {
		return err
	}

	return json.Unmarshal(data, &sm.servers)
}

// saveConfig saves server configurations to JSON file
func (sm *ServerManager) saveConfig() error {
	sm.mu.RLock()
	defer sm.mu.RUnlock()
	return sm.saveConfigLocked()
}

// saveConfigLocked saves config without acquiring lock (internal use)
func (sm *ServerManager) saveConfigLocked() error {
	data, err := json.MarshalIndent(sm.servers, "", "  ")
	if err != nil {
		return err
	}

	return os.WriteFile(sm.configFile, data, 0600)
}

// GetServers returns all server configurations
func (sm *ServerManager) GetServers() []ServerInfo {
	sm.mu.RLock()
	defer sm.mu.RUnlock()

	// Return a copy to prevent external modification
	servers := make([]ServerInfo, len(sm.servers))
	copy(servers, sm.servers)
	return servers
}

// GetServer returns a specific server by ID
func (sm *ServerManager) GetServer(id string) (*ServerInfo, error) {
	sm.mu.RLock()
	defer sm.mu.RUnlock()

	for i, server := range sm.servers {
		if server.ID == id {
			return &sm.servers[i], nil
		}
	}
	return nil, fmt.Errorf("server not found: %s", id)
}

// AddServer adds a new server configuration
func (sm *ServerManager) AddServer(server ServerInfo) error {
	sm.mu.Lock()
	defer sm.mu.Unlock()

	// Generate ID if not provided
	if server.ID == "" {
		server.ID = fmt.Sprintf("server_%d", len(sm.servers)+1)
	}

	// Check if ID already exists
	for _, s := range sm.servers {
		if s.ID == server.ID {
			return fmt.Errorf("server ID already exists: %s", server.ID)
		}
	}

	// 添加服务器
	sm.servers = append(sm.servers, server)
	if err := sm.saveConfigLocked(); err != nil {
		return err
	}

	// 解锁后尝试连接（避免在锁内执行耗时操作）
	sm.mu.Unlock()
	defer sm.mu.Lock()

	// 尝试连接新添加的服务器
	go sm.connectServerByID(server.ID)

	return nil
}

// UpdateServer updates an existing server configuration
func (sm *ServerManager) UpdateServer(server ServerInfo) error {
	sm.mu.Lock()
	defer sm.mu.Unlock()

	for i, s := range sm.servers {
		if s.ID == server.ID {
			sm.servers[i] = server
			return sm.saveConfigLocked()
		}
	}
	return fmt.Errorf("server not found: %s", server.ID)
}

// DeleteServer deletes a server configuration
func (sm *ServerManager) DeleteServer(id string) error {
	log.Printf("DeleteServer called with ID: %s", id)
	
	sm.mu.Lock()
	defer sm.mu.Unlock()

	log.Printf("Current servers count: %d", len(sm.servers))

	found := false
	for i, s := range sm.servers {
		log.Printf("Checking server %d: ID=%s, Name=%s", i, s.ID, s.Name)
		if s.ID == id {
			log.Printf("Found server to delete: ID=%s, Name=%s", s.ID, s.Name)
			
			// 关闭SSH连接
			if s.SSHClient != nil {
				s.SSHClient.Close()
				log.Printf("SSH connection closed for server: %s", s.ID)
			}
			
			// 执行删除操作
			sm.servers = append(sm.servers[:i], sm.servers[i+1:]...)
			log.Printf("Server removed, new count: %d", len(sm.servers))
			
			// 保存配置到文件
			log.Printf("Saving configuration to %s", sm.configFile)
			err := sm.saveConfigLocked()
			if err != nil {
				log.Printf("Error saving config: %v", err)
				return fmt.Errorf("failed to save config after deletion: %v", err)
			}
			log.Printf("Config saved successfully after deletion")
			found = true
			break
		}
	}
	
	if !found {
		log.Printf("Server with ID %s not found", id)
		return fmt.Errorf("server not found: %s", id)
	}
	
	log.Printf("DeleteServer operation completed successfully for ID: %s", id)
	return nil
}

// connectAllServers attempts to connect to all servers
func (sm *ServerManager) connectAllServers() {
	servers := sm.GetServers()
	for _, server := range servers {
		go sm.connectServerByID(server.ID)
	}
}

// connectServerByID attempts to establish an SSH connection for a server by ID
func (sm *ServerManager) connectServerByID(id string) {
	sm.mu.Lock()
	server, err := sm.getServerByIDLocked(id)
	if err != nil {
		sm.mu.Unlock()
		log.Printf("Failed to find server %s: %v", id, err)
		return
	}
	
	// 创建SSH客户端配置
	config := &ssh.ClientConfig{
		User: server.Username,
		Auth: []ssh.AuthMethod{
			ssh.Password(server.Password),
		},
		HostKeyCallback: ssh.InsecureIgnoreHostKey(),
		Timeout:         10 * time.Second,
	}
	
	// 构建地址
	address := fmt.Sprintf("%s:%d", server.IP, server.SSHPort)
	sm.mu.Unlock()
	
	// 尝试连接
	log.Printf("Attempting to connect to server %s (%s)", server.Name, address)
	client, err := ssh.Dial("tcp", address, config)
	
	sm.mu.Lock()
	defer sm.mu.Unlock()
	
	// 再次获取服务器指针（因为在解锁期间可能被修改）
	server, err = sm.getServerByIDLocked(id)
	if err != nil {
		log.Printf("Server %s no longer exists", id)
		return
	}
	
	if err != nil {
		server.IsConnected = false
		server.LastConnErr = err
		log.Printf("Failed to connect to server %s: %v", server.Name, err)
	} else {
		// 关闭旧连接（如果存在）
		if server.SSHClient != nil {
			server.SSHClient.Close()
		}
		
		server.SSHClient = client
		server.IsConnected = true
		server.LastConnErr = nil
		log.Printf("Successfully connected to server %s", server.Name)
	}
}

// getServerByIDLocked finds a server by ID (must be called with lock held)
func (sm *ServerManager) getServerByIDLocked(id string) (*ServerInfo, error) {
	for i := range sm.servers {
		if sm.servers[i].ID == id {
			return &sm.servers[i], nil
		}
	}
	return nil, fmt.Errorf("server not found: %s", id)
}

// RefreshConnections refreshes all SSH connections
func (sm *ServerManager) RefreshConnections() {
	sm.connectAllServers()
}

// GetServerConnectionStatus returns connection status for all servers
func (sm *ServerManager) GetServerConnectionStatus() map[string]bool {
	sm.mu.RLock()
	defer sm.mu.RUnlock()
	
	status := make(map[string]bool)
	for _, server := range sm.servers {
		status[server.ID] = server.IsConnected
	}
	return status
}
