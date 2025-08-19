# Makefile for Spring Auth Session (Rust)

.PHONY: help setup build test lint fmt clean run dev-up dev-down test-api

# Default target
help:
	@echo "Available commands:"
	@echo "  setup     - Set up development environment"
	@echo "  build     - Build the application"
	@echo "  test      - Run tests"
	@echo "  lint      - Run linting (clippy)"
	@echo "  fmt       - Format code"
	@echo "  clean     - Clean build artifacts"
	@echo "  run       - Run the application"
	@echo "  dev-up    - Start development dependencies (PostgreSQL & Redis)"
	@echo "  dev-down  - Stop development dependencies"
	@echo "  test-api  - Test the API endpoints (requires running server)"

# Set up development environment
setup:
	@echo "Setting up development environment..."
	rustup update
	rustup component add clippy rustfmt
	@if [ ! -f .env ]; then cp .env.example .env; fi
	@echo "✅ Development environment ready!"
	@echo "Don't forget to update .env with your database credentials"

# Build the application
build:
	@echo "Building application..."
	cargo build

# Build for release
build-release:
	@echo "Building application for release..."
	cargo build --release

# Run tests
test:
	@echo "Running tests..."
	cargo test

# Run linting
lint:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Run the application
run: build
	@echo "Starting application..."
	cargo run

# Start development dependencies
dev-up:
	@echo "Starting development dependencies..."
	docker-compose up -d
	@echo "Waiting for services to be ready..."
	@sleep 5
	@echo "✅ Development services are running!"
	@echo "PostgreSQL: localhost:5432"
	@echo "Redis: localhost:6379"

# Stop development dependencies
dev-down:
	@echo "Stopping development dependencies..."
	docker-compose down

# Test the API endpoints
test-api:
	@echo "Testing API endpoints..."
	@if ! command -v jq > /dev/null; then \
		echo "❌ jq is required for API testing. Please install it."; \
		exit 1; \
	fi
	./test-api.sh

# Development workflow: start deps, run app, and test
dev: dev-up
	@echo "Starting development server..."
	@echo "Press Ctrl+C to stop"
	cargo run

# Full check: format, lint, test, build
check: fmt lint test build
	@echo "✅ All checks passed!"

# Install the application
install: build-release
	cargo install --path .

# Generate documentation
docs:
	cargo doc --no-deps --open

# Check for security vulnerabilities
audit:
	cargo audit