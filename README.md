# Work Session Tracker

A fullstack Rust application for tracking work sessions with descriptions and tags.

## Features

- ⏱️ Track work sessions with duration
- 📝 Add descriptions to sessions
- 🏷️ Organize sessions with customizable tags
- 🎨 Color-coded tags
- 📊 View session history
- 🌐 Modern web interface built with Yew
- 🚀 Fast backend with Axum
- 🐘 PostgreSQL database

## Tech Stack

- **Backend**: Rust + Axum + SQLx
- **Frontend**: Rust + Yew + Tailwind CSS
- **Database**: PostgreSQL
- **Deployment**: Docker + Docker Compose

## Getting Started

### Prerequisites

- Docker and Docker Compose
- Rust (for local development)

### Quick Start with Docker

1. Clone the repository
2. Run the application:

```bash
docker-compose up --build
```

3. Open your browser and navigate to:
   - Frontend: http://localhost:8000
   - Backend API: http://localhost:8080

### Local Development

#### Backend

```bash
# Install dependencies
cd backend
cargo install sqlx-cli

# Start PostgreSQL (or use Docker)
docker run --name postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=work_tracker -p 5432:5432 -d postgres:15

# Set environment variable
export DATABASE_URL="postgres://postgres:password@localhost:5432/work_tracker"

# Run migrations
sqlx migrate run

# Start the backend
cargo run
```

#### Frontend

```bash
# Install trunk
cargo install trunk

# Install wasm target
rustup target add wasm32-unknown-unknown

# Start the frontend dev server
cd frontend
trunk serve
```

## API Endpoints

### Sessions

- `GET /api/sessions` - Get all sessions
- `POST /api/sessions` - Create a new session
- `GET /api/sessions/:id` - Get a specific session
- `PUT /api/sessions/:id` - Update a session
- `DELETE /api/sessions/:id` - Delete a session

### Tags

- `GET /api/tags` - Get all tags
- `POST /api/tags` - Create a new tag
- `GET /api/tags/:id` - Get a specific tag
- `PUT /api/tags/:id` - Update a tag
- `DELETE /api/tags/:id` - Delete a tag

## Database Schema

### work_sessions
- `id` (UUID, Primary Key)
- `duration_seconds` (Integer)
- `description` (Text, Optional)
- `created_at` (Timestamp)
- `updated_at` (Timestamp)

### tags
- `id` (UUID, Primary Key)
- `name` (String, Unique)
- `color` (String, Optional - hex color)
- `created_at` (Timestamp)

### session_tags (Junction Table)
- `session_id` (UUID, Foreign Key)
- `tag_id` (UUID, Foreign Key)

## Deployment

The application is designed to be deployed with Docker Compose and works well on platforms like Railway, Heroku, or any Docker-compatible hosting service.

### Environment Variables

- `DATABASE_URL` - PostgreSQL connection string
- `RUST_LOG` - Logging level (optional, defaults to "info")

## Development with Claude Code

This project is set up to work with Claude Code for continued development:

```bash
# Navigate to project directory
cd work-session-tracker

# Use Claude Code to make changes
claude-code "Add a new feature to export sessions to CSV"
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test locally
5. Submit a pull request

## License

MIT License - see LICENSE file for details