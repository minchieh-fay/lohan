package main

import (
	"context"
	"log"
)

// App struct
type App struct {
	ctx context.Context
}

// NewApp creates a new App application struct
func NewApp() *App {
	return &App{}
}

// startup is called when the app starts. The context is saved
// so we can call the runtime methods
func (a *App) startup(ctx context.Context) {
	a.ctx = ctx
	log.Println("Lohan AI System Analyst started")
}

// domReady is called after the front-end dom has been loaded
func (a *App) domReady(ctx context.Context) {
	log.Println("Frontend DOM ready")
}

// beforeClose is called when the app is about to quit
func (a *App) beforeClose(ctx context.Context) (prevent bool) {
	log.Println("Lohan AI System Analyst shutting down")
	return false
}

// Greet returns a greeting for the given name
func (a *App) Greet(name string) string {
	return "Hello " + name + ", It's show time!"
}
