# Rust Axum Backend with SQLite

A modern REST API backend built with Rust, Axum web framework, and SQLite database. (Vibe coded)

## 🚀 Features

- **Fast & Lightweight**: Built with Rust for maximum performance
- **Modern Web Framework**: Uses Axum for routing and HTTP handling  
- **SQLite Database**: Embedded database with SQLx for type-safe queries
- **CORS Support**: Configured for frontend integration
- **Modular Architecture**: Clean separation of concerns
- **Static File Serving**: Serves frontend assets from `dist/` directory

## 📋 Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## 🛠️ Installation

1. Clone the repository or navigate to the backend directory
2. Install dependencies:
   ```bash
   cargo build
   ```

## 🚀 Running the Server

Start the development server:
```bash
cargo run
```

The server will start on `http://localhost:3000`

## 📚 API Endpoints

### GET `/api/hello`
Returns a greeting message.

**Response:**
```json
{
  "message": "Hello from Axum backend with SQLite!"
}
```

### GET `/api/users`
Retrieves all users from the database.

**Response:**
```json
[
  {
    "id": 1,
    "name": "Prop"
  },
  {
    "id": 2,
    "name": "Bob"
  }
]
```

### POST `/api/users`
Creates a new user.

**Request Body:**
```json
{
  "name": "John Doe"
}
```

**Response:**
```json
{
  "id": 3,
  "name": "John Doe"
}
```

## 🗂️ Project Structure

```
src/
├── main.rs          # Application entry point and server setup
├── database.rs      # Database initialization and connection
├── handlers.rs      # Route handlers and business logic
├── models.rs        # Data structures and schemas
└── routes.rs        # Route definitions and configuration
```

## 🗄️ Database

The application uses SQLite with the following schema:

### Users Table
```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);
```

The database file (`users.db`) is automatically created on first run with initial seed data.

## 🔧 Configuration

### CORS Settings
- Allowed Origin: `http://localhost:5173` (Vite dev server)
- Allowed Methods: `GET`, `POST`
- Allowed Headers: `Content-Type`

### Server Settings
- Host: `0.0.0.0`
- Port: `3000`

## 🧪 Development

### Building
```bash
cargo build
```

### Running in Development
```bash
cargo run
```

### Checking Code
```bash
cargo check
```

## 📦 Dependencies

- **axum**: Modern web framework
- **tokio**: Async runtime
- **sqlx**: Async SQL toolkit with SQLite support
- **serde**: Serialization/deserialization
- **tower**: Service abstractions and middleware
- **tower-http**: HTTP-specific middleware (CORS, static files)

## 🔒 Environment

The application is configured for development with:
- Debug logging enabled
- CORS configured for local frontend development
- SQLite database for easy local development

## 📝 License

This project is part of a learning/playground repository.
