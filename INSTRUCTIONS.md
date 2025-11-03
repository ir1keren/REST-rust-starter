# INSTRUCTIONS for Claude Sonnet

Create a reusable demo repository that showcases a production-leaning Rust REST API using an MVC architecture (models, controllers, services, repositories, views) with optional OpenAPI docs and CI/Docker. This repo will serve as a portfolio artifact and a base for client projects.

## Objectives
- Ship a clean, documented starter API (CRUD) that clients can run quickly.
- Demonstrate three sellable offerings via branches: REST API starter, OpenAPI documentation, and CI/Docker.
- Enforce MVC separation and good practices: typed errors, logging, CORS, health checks.

## Scope (3 branches)
- main: Minimal REST API (Actix or Ntex) with in-memory repository, MVC separation.
- openapi: Add OpenAPI (utoipa + Swagger UI), stable public API; include request/response validation.
- ci-docker: Add GitHub Actions (fmt, clippy, tests, build) + Dockerfile (+ optional Compose).

Notes:
- Default to Actix-Web unless asked for Ntex; keep structure identical either way.
- Domain: Project (parent) and Task (child, many-to-one). Use UUIDs.

## Tech Stack
- Rust 1.74+ (edition 2021)
- Framework: ntex
- serde / serde_json, uuid, chrono, thiserror
- tracing + tracing-subscriber
- Optional feature `openapi`: utoipa, utoipa-swagger-ui
- Optional (future): sqlx (Postgres), redis, auth (JWT)

## Repository Layout (MVC)
```
src/
  main.rs               # bootstrap, server wiring
  routes.rs             # route registration
  config.rs             # env/config loader
  middleware/           # cors, tracing, error mapping
  models/               # entities + DTOs
    mod.rs
    project.rs
    task.rs
  controllers/          # HTTP handlers (thin)
    mod.rs
    project_controller.rs
    task_controller.rs
  services/             # business logic (no HTTP)
    mod.rs
    project_service.rs
    task_service.rs
  repositories/         # data access (start in-memory)
    mod.rs
    project_repo.rs
    task_repo.rs
  views/                # response helpers, error types, pagination
    mod.rs
    api_response.rs
```

## Implementation Steps

### 1) main branch (starter API)
- Implement models
  - Task { id: Uuid, project_id: Uuid, title: String, done: bool, created_at: DateTime<Utc> }
  - TaskCreate { project_id, title }, TaskUpdate { title?, done? }
  - Project with basic fields (id, name)
- Repositories (in-memory HashMap + RwLock)
  - CRUD for Task and minimal list for Project
- Services
  - Business logic: validation, updates, lookups; return Result<T, ApiError>
- Controllers (thin):
  - GET /tasks, GET /tasks/{id}, POST /tasks, PUT /tasks/{id}, DELETE /tasks/{id}
  - Return JSON (200/201/204); map errors via ResponseError
- App wiring
  - routes.rs configures handlers
  - middleware: CORS, Logger, request-id (optional)
  - health: GET /health (200 OK)
- Logging & errors
  - tracing_subscriber with env filter
  - thiserror-based ApiError implementing ResponseError

Acceptance criteria (main)
- curl GET http://localhost:8080/health returns 200
- CRUD round-trip works for /tasks
- Controllers are thin (no business logic), services call repositories
- Project builds and runs via `cargo run`

### 2) openapi branch (docs + validation)
- Add feature flag `openapi` in Cargo.toml enabling utoipa & swagger-ui.
- Derive ToSchema for DTOs and entity structs.
- Create an OpenApi doc that enumerates task endpoints.
- Add Swagger UI at /docs serving /api-docs/openapi.json.
- Add basic request/response validation where applicable.

Acceptance criteria (openapi)
- http://localhost:8080/docs loads Swagger UI
- /api-docs/openapi.json matches implemented endpoints
- No public API breaking changes from main
- Postman collection exported in `openapi/postman_collection.json`

### 3) ci-docker branch (CI + Docker)
- GitHub Actions workflow `.github/workflows/ci.yml`:
  - runs: format check, clippy (deny warnings), tests, build caching
- Dockerfile (multi-stage):
  - builder (rust:slim), runtime (debian:bookworm-slim)
  - copy binary, set nonroot user, expose 8080
- Optional: docker-compose.yml mapping 8080:8080
- Optional security: cargo-audit/deny, SBOM (syft)

Acceptance criteria (ci-docker)
- CI passes on push/PR
- `docker build` and `docker run -p 8080:8080` serves API

## Key Code Sketches

Models (example)
```rust
// src/models/task.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub done: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct TaskCreate { pub project_id: Uuid, pub title: String }

#[derive(Debug, Deserialize)]
pub struct TaskUpdate { pub title: Option<String>, pub done: Option<bool> }
```

Controller (example, Actix)
```rust
// src/controllers/task_controller.rs
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::services::task_service::TaskService;
use crate::models::task::{TaskCreate, TaskUpdate};

#[get("/tasks")]
pub async fn list(svc: web::Data<TaskService>) -> impl Responder {
    let items = svc.list().await?;
    Ok::<_, actix_web::Error>(HttpResponse::Ok().json(items))
}
```

Error handling
```rust
// src/views/api_response.rs
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorBody { pub code: String, pub message: String }

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("not found")] NotFound { resource: String },
    #[error("bad request: {0}")] BadRequest(String),
    #[error("internal error")] Internal,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode { /* map variants */ StatusCode::INTERNAL_SERVER_ERROR }
    fn error_response(&self) -> HttpResponse { /* return ErrorBody */ HttpResponse::build(self.status_code()).finish() }
}
```

## Run Instructions (Windows)
- Dev:
```powershell
cargo run
```
- Docker:
```powershell
docker build -t rust-mvc-api .
docker run --rm -p 8080:8080 rust-mvc-api
```

## Deliverables
- main: API with MVC, health endpoint, README, Postman collection
- openapi: Swagger UI + OpenAPI JSON + validation + Postman export
- ci-docker: CI workflow + Dockerfile (+ optional compose)

## Constraints & Quality Gates
- Keep controllers thin; logic in services; storage in repositories.
- Typed errors with consistent JSON error body.
- Lint/clippy: no warnings in CI.
- Public API stable between `main` and `openapi` branches.

## Stretch Goals (optional)
- Auth (JWT, refresh), roles (RBAC)
- Persistence: sqlx + Postgres migrations
- Observability: OTEL metrics/traces, Prometheus endpoint
- Real-time: SSE/WebSockets for task updates

## Submission Checklist
- [ ] README with quickstart and endpoints
- [ ] Branches created: main, openapi, ci-docker
- [ ] CI green on push/PR
- [ ] Swagger UI loads; endpoints functional
- [ ] Docker image builds and runs
