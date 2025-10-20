package main

import (
	"fmt"
	"log"
	"time"

	"golang.org/x/crypto/ssh"
)

// SSHExecutor handles SSH connections and command execution
type SSHExecutor struct {
	serverManager *ServerManager
}

// NewSSHExecutor creates a new SSH executor instance
func NewSSHExecutor(serverManager *ServerManager) *SSHExecutor {
	return &SSHExecutor{
		serverManager: serverManager,
	}
}

// createSSHClient creates an SSH client for the specified server
func (se *SSHExecutor) createSSHClient(server *ServerInfo) (*ssh.Client, error) {
	// Configure SSH client
	config := &ssh.ClientConfig{
		User: server.Username,
		Auth: []ssh.AuthMethod{
			ssh.Password(server.Password),
		},
		HostKeyCallback: ssh.InsecureIgnoreHostKey(), // 生产环境应该验证主机密钥
		Timeout:         10 * time.Second,
	}

	// Connect to server
	address := fmt.Sprintf("%s:%d", server.IP, server.SSHPort)
	client, err := ssh.Dial("tcp", address, config)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to %s: %v", address, err)
	}

	return client, nil
}

// executeCommandOnClient executes a command on an existing SSH client
func (se *SSHExecutor) executeCommandOnClient(client *ssh.Client, command string, server *ServerInfo) (string, error) {
	// Create session
	session, err := client.NewSession()
	if err != nil {
		return "", fmt.Errorf("failed to create session: %v", err)
	}
	defer session.Close()

	// If not using root and need sudo/su, wrap the command
	if !server.UseRoot && server.SudoMethod != "" {
		if server.SudoMethod == "sudo" {
			command = fmt.Sprintf("echo '%s' | sudo -S %s", server.SudoPass, command)
		} else if server.SudoMethod == "su" {
			command = fmt.Sprintf("echo '%s' | su -c '%s'", server.SudoPass, command)
		}
	}

	// Execute command
	output, err := session.CombinedOutput(command)
	if err != nil {
		return string(output), fmt.Errorf("command failed: %v", err)
	}

	return string(output), nil
}

// ExecuteCommand executes a command on the specified server
func (se *SSHExecutor) ExecuteCommand(serverID string, command string) (string, error) {
	server, err := se.serverManager.GetServer(serverID)
	if err != nil {
		return "", err
	}

	log.Printf("Executing command on server %s (%s): %s", server.Name, server.IP, command)

	// 获取SSH客户端
	var client *ssh.Client
	var clientNeedsClose bool
	
	if server.SSHClient != nil && server.IsConnected {
		log.Printf("Using existing SSH connection for server %s", server.Name)
		client = server.SSHClient
		clientNeedsClose = false
	} else {
		log.Printf("Creating new SSH connection for server %s", server.Name)
		client, err = se.createSSHClient(server)
		if err != nil {
			return "", err
		}
		clientNeedsClose = true
		defer func() {
			if clientNeedsClose && client != nil {
				client.Close()
			}
		}()
	}

	// Execute command
	result, err := se.executeCommandOnClient(client, command, server)
	if err != nil {
		// 如果使用的是已有连接但执行失败，可能是连接已断开
		if !clientNeedsClose && server.IsConnected {
			log.Printf("Command execution failed with existing connection, marking server %s as disconnected", server.Name)
			se.serverManager.mu.Lock()
			server.IsConnected = false
			se.serverManager.mu.Unlock()
		}
		return "", err
	}

	// Format result
	formattedResult := fmt.Sprintf("=== %s (%s) ===\n$ %s\n%s",
		server.Name, server.IP, command, result)

	return formattedResult, nil
}

// TestConnection tests SSH connection to the specified server
func (se *SSHExecutor) TestConnection(server *ServerInfo) error {
	log.Printf("Testing connection to server %s (%s)", server.Name, server.IP)

	// 尝试使用ServerManager的连接机制进行测试
	se.serverManager.connectServerByID(server.ID)
	
	// 短暂等待连接尝试完成
	time.Sleep(1 * time.Second)
	
	// 再次获取服务器状态
	updatedServer, err := se.serverManager.GetServer(server.ID)
	if err != nil {
		return err
	}
	
	if !updatedServer.IsConnected {
		if updatedServer.LastConnErr != nil {
			return fmt.Errorf("connection test failed: %v", updatedServer.LastConnErr)
		}
		return fmt.Errorf("connection test failed: unknown error")
	}

	log.Printf("Connection test successful for %s", server.Name)
	return nil
}

// ExecuteOnMultipleServers executes a command on multiple servers
func (se *SSHExecutor) ExecuteOnMultipleServers(serverIDs []string, command string) (map[string]string, error) {
	results := make(map[string]string)

	for _, serverID := range serverIDs {
		result, err := se.ExecuteCommand(serverID, command)
		if err != nil {
			results[serverID] = fmt.Sprintf("Error: %v", err)
		} else {
			results[serverID] = result
		}
	}

	return results, nil
}

// GetDiskInfo gets disk information from the specified server
func (se *SSHExecutor) GetDiskInfo(serverID string) (string, error) {
	return se.ExecuteCommand(serverID, "df -h")
}

// GetMemoryInfo gets memory information from the specified server
func (se *SSHExecutor) GetMemoryInfo(serverID string) (string, error) {
	return se.ExecuteCommand(serverID, "free -h")
}

// GetCPUInfo gets CPU information from the specified server
func (se *SSHExecutor) GetCPUInfo(serverID string) (string, error) {
	return se.ExecuteCommand(serverID, "cat /proc/cpuinfo | grep 'model name' | head -n 1")
}

// GetSystemInfo gets general system information from the specified server
func (se *SSHExecutor) GetSystemInfo(serverID string) (string, error) {
	return se.ExecuteCommand(serverID, "uname -a")
}

// GetProcessInfo gets process information from the specified server
func (se *SSHExecutor) GetProcessInfo(serverID string) (string, error) {
	return se.ExecuteCommand(serverID, "ps aux --sort=-%cpu | head -10")
}

// GetNetworkInfo gets network information from the specified server
func (se *SSHExecutor) GetNetworkInfo(serverID string) (string, error) {
	return se.ExecuteCommand(serverID, "ss -tuln")
}

// GetLogInfo gets recent log information from the specified server
func (se *SSHExecutor) GetLogInfo(serverID string, lines int) (string, error) {
	command := fmt.Sprintf("tail -n %d /var/log/syslog", lines)
	return se.ExecuteCommand(serverID, command)
}
