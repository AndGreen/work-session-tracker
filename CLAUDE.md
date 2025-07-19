# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Architecture Overview

This is a fullstack Rust application using a workspace structure:

- **Backend**: Axum web framework with PostgreSQL database via SQLx
- **Frontend**: Yew (Rust WASM) single-page application with Tailwind CSS
- **Shared**: Common types and structures used by both frontend and backend
- **Database**: PostgreSQL with SQLx migrations

The application tracks work sessions with duration, descriptions, and tags. Sessions can have multiple tags (many-to-many relationship).

## Development Commands

### Backend Development
```bash
# Start backend with database
docker-compose up postgres -d
export DATABASE_URL="postgres://postgres:password@localhost:5432/work_tracker"
cd backend
cargo run

# Run migrations
sqlx migrate run

# Database setup for first time
cargo install sqlx-cli
```

### Frontend Development
```bash
# Install prerequisites
cargo install trunk
rustup target add wasm32-unknown-unknown

# Start frontend dev server
cd frontend
trunk serve
```

### Full Stack Development
```bash
# Start entire stack with Docker
docker-compose up --build

# Frontend: http://localhost:8000
# Backend API: http://localhost:8080
```

## Key Architecture Details

### Workspace Structure
- Root `Cargo.toml` defines workspace with shared dependencies
- `shared/` crate contains common types like `WorkSession`, `Tag`, API DTOs
- Backend uses Axum with SQLx for database operations
- Frontend uses Yew with yew-router for SPA routing

### Database Schema
- `work_sessions` table with UUID primary keys
- `tags` table with unique names and optional color
- `session_tags` junction table for many-to-many relationship
- All tables use `TIMESTAMPTZ` for timestamps

### API Routes
Sessions: `/api/sessions` (GET/POST), `/api/sessions/:id` (GET/PUT/DELETE)
Tags: `/api/tags` (GET/POST), `/api/tags/:id` (GET/PUT/DELETE)

### Frontend Routing
- `/` - Home page
- `/sessions` - Sessions list
- `/tags` - Tags management
- `/sessions/:id` - Session detail view

### Key Implementation Details
- **Shared State**: All types are defined in `shared/src/lib.rs` and used by both frontend and backend
- **API Communication**: Frontend uses `gloo-net` for HTTP requests to backend API
- **Database Queries**: Backend uses SQLx with compile-time verified queries
- **CORS**: Backend configured to allow frontend origin for local development
- **Error Handling**: Standardized `ApiResponse<T>` type for consistent API responses
- **Database Relationships**: Sessions and tags linked via `session_tags` junction table with CASCADE delete

## Environment Variables

- `DATABASE_URL` - PostgreSQL connection string
- `RUST_LOG` - Logging level (defaults to "info")

## Testing & Building

Since this is a Rust workspace, use standard Cargo commands:
```bash
# Build entire workspace
cargo build

# Test entire workspace
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Test specific crate
cargo test -p work-session-tracker-backend
cargo test -p work-session-tracker-frontend
cargo test -p shared
```

For frontend specifically:
```bash
# Build frontend for production
cd frontend
trunk build --release

# Watch mode for development (rebuilds on changes to src/ and ../shared/src)
trunk serve
```

## Development Workflow

### Making Database Changes
1. Create new migration: `sqlx migrate add <migration_name>`
2. Edit the migration file in `backend/migrations/`
3. Apply migration: `sqlx migrate run`
4. Update shared types in `shared/src/lib.rs` if needed