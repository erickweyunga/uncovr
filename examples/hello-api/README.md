# Hello API Example

This example demonstrates how to use Uncover framework with configuration management.

## Features

- **Configuration Management**: Uses `meta.rs` for centralized configuration
- **CORS Support**: Environment-based CORS configuration
- **OpenAPI Documentation**: Auto-generated API docs
- **Type-safe Endpoints**: Compile-time checked request/response types

## Configuration

The application configuration is defined in `src/meta.rs`:

```rust
pub fn config() -> AppConfig {
    AppConfig::new("Hello API", "1.0.0")
        .description("A simple example showing user creation with Uncover framework")
        .bind("127.0.0.1:3000")
        .environment(Environment::Development)
        .cors(CorsConfig::development())
        .docs(true)
        .add_server("http://localhost:3000", "Local development")
}
```

### CORS Configuration

**Development Mode** (default):
- Allows all origins (`*`)
- Allows all methods
- Allows all headers
- No credentials required

**Production Mode**:
```rust
.environment(Environment::Production)
.cors(CorsConfig::production(vec![
    "https://yourdomain.com".to_string(),
    "https://www.yourdomain.com".to_string(),
]))
```

## Running the Example

```bash
cargo run
```

The server will start on `http://127.0.0.1:3000` with:
- API endpoint: `POST /users`
- OpenAPI JSON: `http://127.0.0.1:3000/openapi.json`
- Interactive docs: `http://127.0.0.1:3000/docs`

## Testing

### Create a user:
```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'
```

### Test CORS headers:
```bash
curl -X OPTIONS http://localhost:3000/users \
  -H "Origin: http://example.com" \
  -H "Access-Control-Request-Method: POST" \
  -v
```

### View API documentation:
Open `http://127.0.0.1:3000/docs` in your browser.

## Configuration Options

### AppConfig Methods

- `.name(string)` - Application name
- `.version(string)` - API version
- `.description(string)` - API description
- `.bind(address)` - Server bind address
- `.environment(env)` - Environment (Development, Staging, Production)
- `.cors(config)` - CORS configuration
- `.docs(bool)` - Enable/disable OpenAPI documentation
- `.add_server(url, description)` - Add API server URL
- `.env(key, value)` - Add environment variable

### CorsConfig Methods

- `CorsConfig::development()` - Permissive CORS for development
- `CorsConfig::production(origins)` - Restrictive CORS for production
- Custom configuration with builder methods

## Project Structure

```
examples/hello-api/
├── src/
│   ├── main.rs    # Main application with endpoint definitions
│   └── meta.rs    # Configuration file
├── Cargo.toml
└── README.md
```
