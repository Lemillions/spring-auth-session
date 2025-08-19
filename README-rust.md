# Spring Auth Session - Rust Implementation

This is a Rust implementation of the session-based authentication system originally built with Spring Boot. It provides the same functionality using modern Rust web technologies.

## Features

- User registration and authentication
- Session-based authentication with Redis
- Password hashing with Argon2
- PostgreSQL for user data storage
- JWT-like bearer token authentication
- Session management with automatic expiry
- Maximum session limit per user (5 sessions)
- Request tracking (IP address and User-Agent)

## Technology Stack

- **Web Framework**: Axum
- **Database**: PostgreSQL with SQLx
- **Cache/Sessions**: Redis
- **Password Hashing**: Argon2
- **Serialization**: Serde
- **Validation**: Validator
- **Error Handling**: Thiserror + custom error types
- **Configuration**: Environment variables with dotenvy
- **Async Runtime**: Tokio

## API Endpoints

### Authentication

- `POST /auth/register` - Register a new user
- `POST /auth/login` - Login and create a session
- `GET /auth/me` - Get current user info (requires authentication)

### Health Check

- `GET /health` - Health check endpoint

## Quick Start

### Prerequisites

- Rust 1.70+
- PostgreSQL 12+
- Redis 6+

### Development Setup

1. Clone the repository
2. Start the database and Redis services:
   ```bash
   docker-compose up -d
   ```

3. Copy the environment configuration:
   ```bash
   cp .env.example .env
   ```

4. Run the application:
   ```bash
   cargo run
   ```

The server will start on `http://localhost:8080`.

### Testing the API

Register a new user:
```bash
curl -X POST http://localhost:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com", "password": "password123"}'
```

Login:
```bash
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "john@example.com", "password": "password123"}'
```

Get user info:
```bash
curl -X GET http://localhost:8080/auth/me \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

## Configuration

The application uses environment variables for configuration. See `.env.example` for all available options.

### Database

Set `DATABASE_URL` or use individual components:
- `DB_HOST` (default: localhost)
- `DB_PORT` (default: 5432)
- `DB_NAME` (default: session_auth)
- `DB_USER` (default: postgres)
- `DB_PASSWORD` (default: password)

### Redis

Set `REDIS_URL` or use individual components:
- `REDIS_HOST` (default: localhost)
- `REDIS_PORT` (default: 6379)

### Server

- `PORT` (default: 8080)

## Architecture

The application follows Domain-Driven Design principles with clear separation of concerns:

```
src/
├── domain/           # Business logic and entities
├── infrastructure/   # External concerns (database, web, redis)
├── errors/          # Error handling
└── config/          # Configuration management
```

### Key Components

- **User Entity**: Represents a user with email, password, etc.
- **Session Management**: Redis-based session storage with automatic expiry
- **Use Cases**: Login and Register business logic
- **Web Layer**: HTTP handlers, DTOs, extractors, and middleware
- **Database Layer**: User repository with PostgreSQL

## Session Management

Sessions are stored in Redis with the following features:

- 7-day expiry time
- Maximum 5 sessions per user (oldest removed automatically)
- Tracks IP address and User-Agent
- Automatic last-access time updates
- Bearer token authentication

## Error Handling

The application uses structured error handling with appropriate HTTP status codes:

- `400 Bad Request` - Validation errors, duplicate email
- `401 Unauthorized` - Invalid credentials, expired sessions
- `404 Not Found` - User not found
- `500 Internal Server Error` - Database/Redis errors

## Validation

Input validation using the `validator` crate:

- Email format validation
- Password length requirements (6-24 characters)
- Required field validation

## Security

- Passwords are hashed using Argon2 (recommended over BCrypt)
- Sessions use UUIDs as tokens
- Automatic session expiry and cleanup
- Request origin tracking

## Development

### Running Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy
```

### Formatting

```bash
cargo fmt
```

### Building for Production

```bash
cargo build --release
```

## Comparison with Java Spring Boot Version

This Rust implementation provides the same functionality as the original Spring Boot application:

| Feature | Java/Spring Boot | Rust |
|---------|------------------|------|
| Web Framework | Spring MVC | Axum |
| Database ORM | JPA/Hibernate | SQLx |
| Password Hashing | BCrypt | Argon2 |
| Session Storage | Spring Data Redis | redis-rs |
| Validation | Jakarta Validation | validator crate |
| JSON Processing | Jackson | Serde |
| Configuration | Spring Properties | Environment Variables |
| Error Handling | @ControllerAdvice | Custom Error Types |

The Rust version offers:
- Better memory safety and performance
- More explicit error handling
- Compile-time guarantees
- Zero-cost abstractions
- Modern async/await support