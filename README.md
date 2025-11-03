# Rust MVC API Demo

A clean, documented Rust REST API starter using ntex web framework with full MVC architecture, OpenAPI documentation, and containerization support.

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.75+ 
- **Podman** (instead of Docker)
- **Git**

### Running Locally

```bash
# Clone the repository
git clone <your-repo-url>
cd REST-rust

# Install dependencies and run
cargo run
```

The API will be available at `http://localhost:8080`

### API Documentation

- **Swagger UI**: `http://localhost:8080/docs`
- **OpenAPI Spec**: `http://localhost:8080/openapi.json`
- **Health Check**: `http://localhost:8080/health`

## 📚 API Endpoints

### Projects

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/projects` | List all projects |
| `POST` | `/api/projects` | Create a new project |
| `GET` | `/api/projects/{id}` | Get project by ID |
| `PUT` | `/api/projects/{id}` | Update project |
| `DELETE` | `/api/projects/{id}` | Delete project |

### Tasks

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/projects/{project_id}/tasks` | List tasks for project |
| `POST` | `/api/projects/{project_id}/tasks` | Create task in project |
| `GET` | `/api/tasks/{id}` | Get task by ID |
| `PUT` | `/api/tasks/{id}` | Update task |
| `DELETE` | `/api/tasks/{id}` | Delete task |

### System

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check endpoint |
| `GET` | `/docs` | Swagger UI documentation |
| `GET` | `/openapi.json` | OpenAPI specification |

## 🏗️ Architecture

This project follows a clean MVC (Model-View-Controller) architecture:

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library root
├── config.rs            # Configuration management
├── routes.rs            # Route definitions & OpenAPI setup
├── models/              # Data models
│   ├── mod.rs
│   ├── project.rs       # Project model
│   └── task.rs          # Task model
├── controllers/         # HTTP request handlers
│   ├── mod.rs
│   ├── project_controller.rs
│   ├── task_controller.rs
│   └── health_controller.rs
├── services/            # Business logic
│   ├── mod.rs
│   ├── project_service.rs
│   └── task_service.rs
├── repositories/        # Data access layer
│   ├── mod.rs
│   ├── project_repository.rs
│   └── task_repository.rs
└── middleware/          # HTTP middleware
    ├── mod.rs
    ├── cors.rs
    └── logging.rs
```

## 🐳 Container Deployment

### Build with Podman

```bash
# Build the container image
podman build -t rust-mvc-api:latest -f Podmanfile .

# Run the container
podman run -d --name rust-api -p 8080:8080 rust-mvc-api:latest

# Check logs
podman logs rust-api

# Stop and remove
podman stop rust-api
podman rm rust-api
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `SERVER_HOST` | `127.0.0.1` | Server bind address |
| `SERVER_PORT` | `8080` | Server port |
| `RUST_LOG` | `info` | Log level |

### Production Deployment

```bash
# Run with custom configuration
podman run -d \
  --name rust-api \
  -p 8080:8080 \
  -e SERVER_HOST=0.0.0.0 \
  -e SERVER_PORT=8080 \
  -e RUST_LOG=warn \
  rust-mvc-api:latest
```

## 🧪 Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test models::
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for issues
cargo clippy -- -D warnings
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# With specific features
cargo build --features openapi
```

## 📖 Branch Structure

This repository has three main branches demonstrating progressive feature implementation:

### `main` Branch
- ✅ Complete MVC architecture
- ✅ CRUD operations for Projects and Tasks
- ✅ In-memory storage with thread-safe HashMap
- ✅ Error handling and validation
- ✅ Structured logging with tracing
- ✅ CORS middleware
- ✅ Health check endpoint

### `openapi` Branch
- ✅ All features from `main`
- ✅ OpenAPI 3.0 specification with utoipa
- ✅ Swagger UI documentation at `/docs`
- ✅ Schema documentation for all models
- ✅ Comprehensive API documentation

### `ci-docker` Branch (Current)
- ✅ All features from `openapi`
- ✅ GitHub Actions CI/CD pipeline
- ✅ Podman containerization (instead of Docker)
- ✅ Automated testing and building
- ✅ Code quality checks (format, clippy, tests)

## 🔧 Configuration

### Environment Setup

Create a `.env` file for local development:

```env
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
RUST_LOG=debug
```

### Feature Flags

The project supports conditional compilation features:

```toml
# Enable OpenAPI documentation
cargo build --features openapi

# Default features (includes openapi)
cargo build
```

## 📝 Example Usage

### Create a Project

```bash
curl -X POST http://localhost:8080/api/projects \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Project",
    "description": "A sample project"
  }'
```

### Create a Task

```bash
curl -X POST http://localhost:8080/api/projects/{project_id}/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Implement feature",
    "description": "Add new functionality",
    "priority": "High"
  }'
```

### Get All Projects

```bash
curl http://localhost:8080/api/projects
```

## 🚦 CI/CD Pipeline

The GitHub Actions workflow automatically:

1. **Code Quality Checks**:
   - Rust formatting (`cargo fmt`)
   - Linting (`cargo clippy`)
   - Unit tests (`cargo test`)

2. **Build Verification**:
   - Release build (`cargo build --release`)
   - Container build with Podman
   - Container testing (health and API endpoints)

3. **Artifacts**:
   - Container image saved as build artifact
   - Available for download for 7 days

## 🛠️ Technology Stack

- **Framework**: [ntex](https://github.com/ntex-rs/ntex) - Modern async web framework
- **Serialization**: [serde](https://serde.rs/) - JSON serialization/deserialization
- **OpenAPI**: [utoipa](https://github.com/juhaku/utoipa) - OpenAPI documentation generation
- **Logging**: [tracing](https://github.com/tokio-rs/tracing) - Structured logging
- **Error Handling**: [thiserror](https://github.com/dtolnay/thiserror) - Error derive macros
- **UUID**: [uuid](https://github.com/uuid-rs/uuid) - UUID generation
- **Time**: [chrono](https://github.com/chronotope/chrono) - Date and time handling
- **Container**: [Podman](https://podman.io/) - Container management (Docker alternative)

## 📋 TODO / Future Enhancements

- [ ] Database integration (PostgreSQL/MySQL)
- [ ] Authentication and authorization
- [ ] Rate limiting middleware
- [ ] Request validation middleware
- [ ] Metrics and monitoring
- [ ] Database migrations
- [ ] Integration tests
- [ ] Performance benchmarks
- [ ] Configuration file support
- [ ] Graceful shutdown handling

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎯 Project Goals

This project serves as a **clean, documented starter API** that demonstrates:

- ✅ **Modern Rust web development** with async/await
- ✅ **Clean architecture** with separation of concerns
- ✅ **Comprehensive documentation** with OpenAPI/Swagger
- ✅ **Production-ready features** (logging, error handling, CORS)
- ✅ **Container deployment** with Podman
- ✅ **CI/CD pipeline** with automated testing
- ✅ **Code quality** enforcement with formatting and linting

Perfect for developers who want to **quickly bootstrap** a new Rust web API project with industry best practices!