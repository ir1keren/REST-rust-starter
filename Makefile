# Rust MVC API - Makefile
# Cross-platform build automation

.PHONY: help build test run clean fmt clippy check docs podman-build podman-run podman-test docker-build docker-run docker-test container-build container-run container-test

# Default target
help:
	@echo "Rust MVC API - Make Commands"
	@echo ""
	@echo "Development:"
	@echo "  build         - Build the project"
	@echo "  test          - Run tests"
	@echo "  run           - Run the server locally"
	@echo "  check         - Run all quality checks"
	@echo "  fmt           - Format code"
	@echo "  clippy        - Run clippy linter"
	@echo "  docs          - Generate and open documentation"
	@echo ""
	@echo "Container (Podman):"
	@echo "  podman-build  - Build container image with Podman"
	@echo "  podman-run    - Run container with Podman"
	@echo "  podman-test   - Test container endpoints with Podman"
	@echo ""
	@echo "Container (Docker):"
	@echo "  docker-build  - Build container image with Docker"
	@echo "  docker-run    - Run container with Docker"
	@echo "  docker-test   - Test container endpoints with Docker"
	@echo ""
	@echo "Container (Default to Podman):"
	@echo "  container-build - Build container image"
	@echo "  container-run   - Run container"
	@echo "  container-test  - Test container endpoints"
	@echo ""
	@echo "Maintenance:"
	@echo "  clean         - Clean build artifacts"

# Development targets
build:
	@echo "🔨 Building project..."
	cargo build

test:
	@echo "🧪 Running tests..."
	cargo test

run:
	@echo "🚀 Starting server..."
	cargo run

check: fmt clippy test
	@echo "✅ All quality checks passed!"

fmt:
	@echo "🎨 Formatting code..."
	cargo fmt --all

clippy:
	@echo "📎 Running clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

docs:
	@echo "📚 Generating documentation..."
	cargo doc --open

# Container targets (Podman)
podman-build:
	@echo "🐳 Building container image with Podman..."
	podman build -t rust-mvc-api:latest -f Podmanfile .

podman-run:
	@echo "🚀 Starting container with Podman..."
	podman run -d --name rust-api -p 8080:8080 \
		-e SERVER_HOST=0.0.0.0 \
		-e SERVER_PORT=8080 \
		-e RUST_LOG=info \
		rust-mvc-api:latest
	@echo "✅ Container started on http://localhost:8080"

podman-test:
	@echo "🧪 Testing container with Podman..."
	@sleep 5  # Wait for container to start
	curl -f http://localhost:8080/health
	curl -f http://localhost:8080/api/v1/projects
	@echo "✅ Container tests passed!"

# Container targets (Docker)
docker-build:
	@echo "🐳 Building container image with Docker..."
	docker build -t rust-mvc-api:latest -f Dockerfile .

docker-run:
	@echo "🚀 Starting container with Docker..."
	docker run -d --name rust-api -p 8080:8080 \
		-e SERVER_HOST=0.0.0.0 \
		-e SERVER_PORT=8080 \
		-e RUST_LOG=info \
		rust-mvc-api:latest
	@echo "✅ Container started on http://localhost:8080"

docker-test:
	@echo "🧪 Testing container with Docker..."
	@sleep 5  # Wait for container to start
	curl -f http://localhost:8080/health
	curl -f http://localhost:8080/api/v1/projects
	@echo "✅ Container tests passed!"

# Convenience targets (default to Podman)
container-build: podman-build
container-run: podman-run
container-test: podman-test

# Maintenance targets
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	@echo "✅ Clean complete!"

# Install development dependencies
install-deps:
	@echo "📦 Installing development dependencies..."
	rustup component add rustfmt clippy
	@echo "✅ Dependencies installed!"

# Release build
release:
	@echo "🎁 Building release version..."
	cargo build --release

# Quick development cycle
dev: fmt clippy test run

# CI simulation
ci: fmt clippy test build
	@echo "✅ CI simulation complete!"