# Uncovr Framework Benchmarks

This directory contains benchmarks for the Uncovr web framework, measuring performance across various common API scenarios.

## Prerequisites

The benchmarks use [rewrk](https://github.com/lnx-search/rewrk), a modern HTTP benchmarking tool written in Rust.

### Installing rewrk

```bash
cargo install rewrk --git https://github.com/ChillFish8/rewrk.git
```

Or, on CI environments, rewrk will be installed automatically.

## Running Benchmarks

### Run All Benchmarks

```bash
cargo bench
```

### Run Specific Benchmarks

You can run specific benchmarks by passing their names:

```bash
# Run only basic scenarios
cargo bench -- minimal-ping basic-json

# Run JSON-related benchmarks
cargo bench -- echo-json complex-json nested-json

# Run HTTP method benchmarks
cargo bench -- http-put http-patch http-delete

# Run routing benchmarks
cargo bench -- deep-routing many-endpoints

# Run configuration benchmarks
cargo bench -- with-cors-logging with-openapi-docs
```

## Benchmark Scenarios

The benchmark suite includes 17 scenarios covering different aspects of the framework:

### Quick Reference

| Benchmark | Category | Method | Purpose |
|-----------|----------|--------|---------|
| minimal-ping | Basic | GET | Base framework overhead |
| basic-json | Basic | GET | JSON serialization |
| receive-json | Basic | POST | JSON deserialization |
| echo-json | Basic | POST | Full JSON round-trip |
| complex-json | Complex JSON | POST | Realistic complex payloads |
| json-array | Complex JSON | POST | JSON arrays with objects |
| nested-json | Complex JSON | POST | Nested JSON structures |
| large-json-1kb | Complex JSON | POST | Large payload handling |
| http-put | HTTP Methods | PUT | PUT request handling |
| http-patch | HTTP Methods | PATCH | PATCH request handling |
| http-delete | HTTP Methods | DELETE | DELETE request handling |
| multiple-endpoints | Routing | POST | Multiple route overhead |
| deep-routing | Routing | POST | Deeply nested paths |
| many-endpoints | Routing | POST | Real-world API simulation |
| with-cors-logging | Config | GET | Middleware overhead |
| with-openapi-docs | Config | GET | OpenAPI overhead |

### Basic Scenarios

#### 1. **minimal-ping**
- **Method**: GET
- **Response**: Plain text "pong"
- **Purpose**: Measures the base framework overhead with no JSON processing

#### 2. **basic-json**
- **Method**: GET
- **Response**: JSON object with a single string field
- **Purpose**: Tests basic JSON serialization performance

#### 3. **receive-json**
- **Method**: POST
- **Request**: JSON with name and value fields
- **Response**: Plain text "ok"
- **Purpose**: Tests JSON deserialization performance

#### 4. **echo-json**
- **Method**: POST
- **Request**: JSON with text field
- **Response**: JSON echoing the input text
- **Purpose**: Tests complete JSON serialization/deserialization cycle

### Complex JSON Scenarios

#### 5. **complex-json**
- **Method**: POST
- **Request**: JSON with string, number, boolean, and array fields
- **Response**: JSON with all input fields plus an ID
- **Purpose**: Tests performance with realistic complex payloads

#### 6. **json-array**
- **Method**: POST
- **Request**: JSON array with 3 objects containing id and name fields
- **Response**: JSON with item count
- **Purpose**: Tests handling of JSON arrays with structured objects

#### 7. **nested-json**
- **Method**: POST
- **Request**: Deeply nested JSON structure (user -> profile/settings)
- **Response**: Plain text "ok"
- **Purpose**: Tests deserialization of complex nested structures

#### 8. **large-json-1kb**
- **Method**: POST
- **Request**: ~1KB JSON payload with large string field
- **Response**: JSON with size information
- **Purpose**: Tests performance with larger payloads

### HTTP Method Scenarios

#### 9. **http-put**
- **Method**: PUT
- **Request**: JSON with id and name fields
- **Response**: Plain text "updated"
- **Purpose**: Tests PUT request handling

#### 10. **http-patch**
- **Method**: PATCH
- **Request**: JSON with name field
- **Response**: Plain text "patched"
- **Purpose**: Tests PATCH request handling

#### 11. **http-delete**
- **Method**: DELETE
- **Request**: JSON with id field
- **Response**: Plain text "deleted"
- **Purpose**: Tests DELETE request handling

### Routing Scenarios

#### 12. **multiple-endpoints**
- **Method**: POST (targeting /users)
- **Endpoints**: /ping, /users, /health
- **Purpose**: Tests routing overhead with multiple registered endpoints

#### 13. **deep-routing**
- **Method**: POST
- **Path**: /api/v1/users/123/posts/456/comments
- **Purpose**: Tests performance with deeply nested route paths

#### 14. **many-endpoints**
- **Method**: POST (targeting /api/users)
- **Endpoints**: 20 different REST API endpoints
- **Purpose**: Simulates a real-world API with many routes

### Configuration Scenarios

#### 15. **with-cors-logging**
- **Method**: GET
- **Config**: CORS + logging middleware enabled
- **Purpose**: Measures overhead of middleware layers

#### 16. **with-openapi-docs**
- **Method**: GET
- **Config**: OpenAPI documentation generation enabled
- **Purpose**: Measures overhead of OpenAPI documentation

## Benchmark Configuration

- **Connections**: 10 concurrent connections
- **Threads**: 10 threads
- **Duration**: 
  - Development: 10 seconds
  - CI: 1 second (to reduce CI time)

## Understanding Results

rewrk provides the following metrics:

- **Requests/sec**: Total throughput
- **Transfer/sec**: Data transfer rate
- **Latency**: Request latency statistics (min, max, avg, p50, p95, p99)
- **Total Requests**: Total number of requests completed
- **Total Transfer**: Total data transferred

### Example Output

```
Running "minimal-ping" benchmark
  Connections: 10
  Threads: 10
  Duration: 10s
  
  Total Requests: 1,234,567
  Requests/sec: 123,456.78
  Transfer/sec: 12.34 MB
  
  Latency:
    Min: 45 µs
    Max: 5.2 ms
    Avg: 78 µs
    p50: 72 µs
    p95: 125 µs
    p99: 245 µs
```

## Comparing with Other Frameworks

To compare Uncovr's performance with raw Axum or other frameworks, you can:

1. Run these benchmarks
2. Adapt the benchmark scenarios to other frameworks
3. Compare the requests/sec and latency metrics

The benchmarks are designed to be similar to [Axum's official benchmarks](https://github.com/tokio-rs/axum/blob/main/axum/benches/routing.rs) for easier comparison.

## Tips for Accurate Benchmarking

1. **Close Other Applications**: Minimize background processes during benchmarking
2. **Consistent Hardware**: Run benchmarks on the same machine for comparisons
3. **Multiple Runs**: Run benchmarks multiple times and average the results
4. **System Load**: Avoid running benchmarks under heavy system load
5. **Network**: Tests run on localhost (127.0.0.1) to minimize network variance

## Adding New Benchmarks

To add a new benchmark scenario:

1. Open `benches/framework.rs`
2. Add a new benchmark using the `benchmark()` builder:

```rust
benchmark("my-new-test")
    .method("post")  // optional, defaults to GET
    .path("/custom")  // optional, defaults to /
    .headers(&[("content-type", "application/json")])  // optional
    .body(r#"{"key": "value"}"#)  // optional
    .run(|| {
        // Your endpoint setup here
        Server::new()
            .with_config(minimal_config())
            .register(YourEndpoint)
            .build()
    });
```

## Troubleshooting

### rewrk not found

If you get a "rewrk not found" error:

```bash
cargo install rewrk --git https://github.com/ChillFish8/rewrk.git
```

### Port already in use

The benchmarks use random available ports (binding to `0.0.0.0:0`), so port conflicts should be rare. If you encounter issues, make sure no other benchmarks are running.

### Low performance numbers

If you see unexpectedly low numbers:
- Check system resource usage
- Ensure you're running in release mode (benchmarks should use `--release`)
- Close resource-intensive applications
- Try increasing the benchmark duration for more stable results
