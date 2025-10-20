package main

import (
	"context"
	"fmt"
	"log"
)

// App struct
type App struct {
	ctx           context.Context
	modelManager  *ModelManager
	llmService    *LLMService
	serverManager *ServerManager
	sshExecutor   *SSHExecutor
}

// NewApp creates a new App application struct
func NewApp() *App {
	serverManager := NewServerManager()
	return &App{
		modelManager:  NewModelManager(),
		llmService:    NewLLMService(),
		serverManager: serverManager,
		sshExecutor:   NewSSHExecutor(serverManager),
	}
}

// startup is called when the app starts. The context is saved
// so we can call the runtime methods
func (a *App) startup(ctx context.Context) {
	a.ctx = ctx
	log.Println("Lohan AI System Analyst started")

	// Check if model exists and try to load it
	if a.modelManager.CheckModelExists() {
		modelPath := a.modelManager.GetModelPath()
		if modelPath != "" {
			log.Printf("Found model: %s", modelPath)
			err := a.llmService.LoadModel(modelPath)
			if err != nil {
				log.Printf("Failed to load model: %v", err)
			} else {
				log.Println("Model loaded successfully!")
			}
		}
	} else {
		log.Println("No model found. Please download a model first.")
	}
}

// domReady is called after the front-end dom has been loaded
func (a *App) domReady(ctx context.Context) {
	log.Println("Frontend DOM ready")
}

// beforeClose is called when the app is about to quit
func (a *App) beforeClose(ctx context.Context) (prevent bool) {
	log.Println("Lohan AI System Analyst shutting down")

	// Unload model if loaded
	if a.llmService.IsLoaded() {
		err := a.llmService.UnloadModel()
		if err != nil {
			log.Printf("Error unloading model: %v", err)
		}
	}

	return false
}

// Greet returns a greeting for the given name
func (a *App) Greet(name string) string {
	return "Hello " + name + ", It's show time!"
}

// CheckModelExists checks if any model file exists
func (a *App) CheckModelExists() bool {
	return a.modelManager.CheckModelExists()
}

// GetAvailableModels returns list of available models for download
func (a *App) GetAvailableModels() []ModelInfo {
	return a.modelManager.GetAvailableModels()
}

// DownloadModel downloads a specific model
func (a *App) DownloadModel(modelName string) error {
	return a.modelManager.DownloadModel(modelName, func(percent int) {
		// 这里可以通过事件系统发送进度更新到前端
		// 暂时先打印到控制台
		fmt.Printf("Download progress: %d%%\n", percent)
	})
}

// GetModelPath returns the path to the first available model file
func (a *App) GetModelPath() string {
	return a.modelManager.GetModelPath()
}

// IsModelLoaded returns whether a model is currently loaded
func (a *App) IsModelLoaded() bool {
	return a.llmService.IsLoaded()
}

// GetLoadedModelPath returns the path of the currently loaded model
func (a *App) GetLoadedModelPath() string {
	return a.llmService.GetModelPath()
}

// LoadModel loads a model from the specified path
func (a *App) LoadModel(modelPath string) error {
	return a.llmService.LoadModel(modelPath)
}

// UnloadModel unloads the current model
func (a *App) UnloadModel() error {
	return a.llmService.UnloadModel()
}

// Ask sends a question to the loaded model
func (a *App) Ask(question string) (string, error) {
	if !a.llmService.IsLoaded() {
		return "", fmt.Errorf("no model loaded")
	}
	return a.llmService.Inference(question, 1000)
}

// AnalyzeSystemLog analyzes system log with the loaded model
func (a *App) AnalyzeSystemLog(logContent string) (string, error) {
	if !a.llmService.IsLoaded() {
		return "", fmt.Errorf("no model loaded")
	}
	return a.llmService.AnalyzeSystemLog(logContent)
}

// AnalyzeNetworkCapture analyzes network capture data
func (a *App) AnalyzeNetworkCapture(captureData string) (string, error) {
	if !a.llmService.IsLoaded() {
		return "", fmt.Errorf("no model loaded")
	}
	return a.llmService.AnalyzeNetworkCapture(captureData)
}

// ========== Server Management APIs ==========

// GetServers returns all server configurations
func (a *App) GetServers() []ServerInfo {
	return a.serverManager.GetServers()
}

// GetServer returns a specific server by ID
func (a *App) GetServer(id string) (*ServerInfo, error) {
	return a.serverManager.GetServer(id)
}

// AddServer adds a new server configuration
func (a *App) AddServer(server ServerInfo) error {
	return a.serverManager.AddServer(server)
}

// UpdateServer updates an existing server configuration
func (a *App) UpdateServer(server ServerInfo) error {
	return a.serverManager.UpdateServer(server)
}

// DeleteServer deletes a server configuration
func (a *App) DeleteServer(id string) error {
	log.Printf("App.DeleteServer called with ID: %s", id)
	err := a.serverManager.DeleteServer(id)
	if err != nil {
		log.Printf("App.DeleteServer error: %v", err)
	} else {
		log.Printf("App.DeleteServer success")
	}
	return err
}

// GetServerConnectionStatus returns connection status for all servers
func (a *App) GetServerConnectionStatus() map[string]bool {
	return a.serverManager.GetServerConnectionStatus()
}

// RefreshServerConnections refreshes all SSH connections
func (a *App) RefreshServerConnections() {
	log.Println("Refreshing all server connections")
	a.serverManager.RefreshConnections()
}

// ========== SSH Execution APIs ==========

// ExecuteCommand executes a command on the specified server
func (a *App) ExecuteCommand(serverID string, command string) (string, error) {
	return a.sshExecutor.ExecuteCommand(serverID, command)
}

// ExecuteOnMultipleServers executes a command on multiple servers
func (a *App) ExecuteOnMultipleServers(serverIDs []string, command string) (map[string]string, error) {
	return a.sshExecutor.ExecuteOnMultipleServers(serverIDs, command)
}

// GetDiskInfo gets disk information from the specified server
func (a *App) GetDiskInfo(serverID string) (string, error) {
	return a.sshExecutor.GetDiskInfo(serverID)
}

// GetMemoryInfo gets memory information from the specified server
func (a *App) GetMemoryInfo(serverID string) (string, error) {
	return a.sshExecutor.GetMemoryInfo(serverID)
}

// GetCPUInfo gets CPU information from the specified server
func (a *App) GetCPUInfo(serverID string) (string, error) {
	return a.sshExecutor.GetCPUInfo(serverID)
}

// GetSystemInfo gets general system information from the specified server
func (a *App) GetSystemInfo(serverID string) (string, error) {
	return a.sshExecutor.GetSystemInfo(serverID)
}

// TestConnection tests SSH connection to the specified server
func (a *App) TestConnection(server ServerInfo) error {
	return a.sshExecutor.TestConnection(&server)
}

// GetProcessInfo gets process information from the specified server
func (a *App) GetProcessInfo(serverID string) (string, error) {
	return a.sshExecutor.GetProcessInfo(serverID)
}

// GetNetworkInfo gets network information from the specified server
func (a *App) GetNetworkInfo(serverID string) (string, error) {
	return a.sshExecutor.GetNetworkInfo(serverID)
}

// GetLogInfo gets recent log information from the specified server
func (a *App) GetLogInfo(serverID string, lines int) (string, error) {
	return a.sshExecutor.GetLogInfo(serverID, lines)
}
