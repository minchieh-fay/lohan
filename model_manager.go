package main

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

// ModelInfo represents information about a downloadable model
type ModelInfo struct {
	Name        string `json:"name"`
	Size        string `json:"size"`
	Description string `json:"description"`
	URL         string `json:"url"`
	Filename    string `json:"filename"`
}

// ModelManager handles model download and management
type ModelManager struct {
	modelsDir string
	models    []ModelInfo
}

// getModelBaseURL returns the base URL for model downloads
// Supports environment variable HF_ENDPOINT for custom mirror
func getModelBaseURL() string {
	// Check if custom mirror is set
	if mirror := os.Getenv("HF_ENDPOINT"); mirror != "" {
		return mirror
	}
	// Default to Hugging Face official
	return "https://huggingface.co"
}

// NewModelManager creates a new ModelManager instance
func NewModelManager() *ModelManager {
	modelsDir := "./models"
	os.MkdirAll(modelsDir, 0755)

	baseURL := getModelBaseURL()

	return &ModelManager{
		modelsDir: modelsDir,
		models: []ModelInfo{
			{
				Name:        "千问Code-2.5B",
				Size:        "1.8GB",
				Description: "轻量级代码分析模型，适合系统诊断和网络分析",
				URL:         fmt.Sprintf("%s/Qwen/Qwen-Code-2B-Instruct-GGUF/resolve/main/qwen-code-2b-instruct-q4_k_m.gguf", baseURL),
				Filename:    "qwen-code-2.5b-instruct-q4_k_m.gguf",
			},
			{
				Name:        "千问Code-4B",
				Size:        "3.0GB",
				Description: "中等级别代码分析模型，平衡性能和资源使用",
				URL:         fmt.Sprintf("%s/Qwen/Qwen-Code-4B-Instruct-GGUF/resolve/main/qwen-code-4b-instruct-q4_k_m.gguf", baseURL),
				Filename:    "qwen-code-4b-instruct-q4_k_m.gguf",
			},
			{
				Name:        "千问Code-7B",
				Size:        "5.0GB",
				Description: "高性能代码分析模型，适合复杂的系统分析和调试任务",
				URL:         fmt.Sprintf("%s/Qwen/Qwen-Code-7B-Instruct-GGUF/resolve/main/qwen-code-7b-instruct-q4_k_m.gguf", baseURL),
				Filename:    "qwen-code-7b-instruct-q4_k_m.gguf",
			},
		},
	}
}

// CheckModelExists checks if any model file exists in the models directory
func (mm *ModelManager) CheckModelExists() bool {
	files, err := os.ReadDir(mm.modelsDir)
	if err != nil {
		return false
	}

	for _, file := range files {
		if strings.HasSuffix(file.Name(), ".gguf") {
			return true
		}
	}
	return false
}

// GetAvailableModels returns list of available models for download
func (mm *ModelManager) GetAvailableModels() []ModelInfo {
	return mm.models
}

// DownloadModel downloads a specific model
func (mm *ModelManager) DownloadModel(modelName string, progressCallback func(percent int)) error {
	var selectedModel *ModelInfo
	for _, model := range mm.models {
		if model.Name == modelName {
			selectedModel = &model
			break
		}
	}

	if selectedModel == nil {
		return fmt.Errorf("model %s not found", modelName)
	}

	filePath := filepath.Join(mm.modelsDir, selectedModel.Filename)
	tmpFilePath := filePath + ".tmp"

	// Check if file already exists
	if _, err := os.Stat(filePath); err == nil {
		return fmt.Errorf("model file already exists")
	}

	// Remove any existing temporary file
	os.Remove(tmpFilePath)

	// Create the temporary file
	out, err := os.Create(tmpFilePath)
	if err != nil {
		return err
	}
	defer out.Close()

	// Download the file
	resp, err := http.Get(selectedModel.URL)
	if err != nil {
		os.Remove(tmpFilePath) // Clean up on error
		return err
	}
	defer resp.Body.Close()

	// Check HTTP response status
	if resp.StatusCode != http.StatusOK {
		os.Remove(tmpFilePath)
		return fmt.Errorf("download failed: HTTP %d", resp.StatusCode)
	}

	// Get the content length
	contentLength := resp.ContentLength

	// Create a progress reader
	progressReader := &ProgressReader{
		Reader:     resp.Body,
		Total:      contentLength,
		OnProgress: progressCallback,
	}

	// Copy the content
	_, err = io.Copy(out, progressReader)
	if err != nil {
		os.Remove(tmpFilePath) // Clean up on error
		return err
	}

	// Close the file before renaming
	out.Close()

	// Rename temporary file to final filename
	err = os.Rename(tmpFilePath, filePath)
	if err != nil {
		os.Remove(tmpFilePath) // Clean up on error
		return fmt.Errorf("failed to rename file: %v", err)
	}

	return nil
}

// GetModelPath returns the path to the first available model file
func (mm *ModelManager) GetModelPath() string {
	files, err := os.ReadDir(mm.modelsDir)
	if err != nil {
		return ""
	}

	for _, file := range files {
		if strings.HasSuffix(file.Name(), ".gguf") {
			return filepath.Join(mm.modelsDir, file.Name())
		}
	}
	return ""
}

// ProgressReader wraps an io.Reader to track download progress
type ProgressReader struct {
	Reader     io.Reader
	Total      int64
	ReadBytes  int64
	OnProgress func(percent int)
}

func (pr *ProgressReader) Read(p []byte) (n int, err error) {
	n, err = pr.Reader.Read(p)
	pr.ReadBytes += int64(n)

	if pr.Total > 0 && pr.OnProgress != nil {
		percent := int((pr.ReadBytes * 100) / pr.Total)
		pr.OnProgress(percent)
	}

	return n, err
}
