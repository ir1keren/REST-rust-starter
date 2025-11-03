# Rust MVC API - Makefile
# Cross-platform build automation

.PHONY: help build test run clean fmt clippy check docs container-build container-run container-test

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

# Container targets
container-build:
	@echo "🐳 Building container image..."
	podman build -t rust-mvc-api:latest -f Podmanfile .

container-run:
	@echo "🚀 Starting container..."
	podman run -d --name rust-api -p 8080:8080 \
		-e SERVER_HOST=0.0.0.0 \
		-e SERVER_PORT=8080 \
		-e RUST_LOG=info \
		rust-mvc-api:latest
	@echo "✅ Container started on http://localhost:8080"

container-test:
	@echo "🧪 Testing container..."
	@sleep 5  # Wait for container to start
	curl -f http://localhost:8080/health
	curl -f http://localhost:8080/api/projects
	@echo "✅ Container tests passed!"

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