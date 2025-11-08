//! Framework benchmarks for Uncovr
//!
//! This benchmark suite tests the performance characteristics of the Uncovr framework
//! across different scenarios. Each benchmark is focused on a specific aspect of
//! framework performance.
//!
//! ## Benchmarks
//!
//! 1. **ping** - Minimal overhead baseline (no JSON, simple string response)
//! 2. **json_simple** - Basic JSON serialization/deserialization
//! 3. **json_complex** - Complex nested JSON structures
//! 4. **large_payload** - Large request/response handling (1KB payload)
//! 5. **route_matching** - Deep nested route performance
//!
//! ## Running Benchmarks
//!
//! ```bash
//! cargo bench
//! ```

use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use uncovr::prelude::*;
use uncovr::server::endpoint::{Docs, Endpoint, Route};

// =============================================================================
// Benchmark 1: Minimal Overhead (Ping)
// =============================================================================

#[derive(Clone)]
struct Ping;

impl Endpoint for Ping {
    fn ep(&self) -> Route {
        Route::GET("/ping")
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new().summary("Minimal overhead ping endpoint"))
    }
}

#[async_trait]
impl API for Ping {
    type Req = ();
    type Res = &'static str;

    async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
        "pong"
    }
}

// =============================================================================
// Benchmark 2: Simple JSON
// =============================================================================

#[derive(Clone)]
struct SimpleJson;

#[derive(Serialize, JsonSchema)]
struct SimpleResponse {
    message: String,
    timestamp: u64,
}

impl Endpoint for SimpleJson {
    fn ep(&self) -> Route {
        Route::GET("/api/simple")
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new().summary("Simple JSON serialization"))
    }
}

#[async_trait]
impl API for SimpleJson {
    type Req = ();
    type Res = Json<SimpleResponse>;

    async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
        Json(SimpleResponse {
            message: "Hello, World!".to_string(),
            timestamp: 1234567890,
        })
    }
}

// =============================================================================
// Benchmark 3: Complex JSON
// =============================================================================

#[derive(Clone)]
struct ComplexJson;

#[derive(Default, Deserialize, JsonSchema)]
struct ComplexRequest {
    name: String,
    email: String,
    age: u32,
    active: bool,
    tags: Vec<String>,
}

#[derive(Serialize, JsonSchema)]
struct ComplexResponse {
    id: u64,
    name: String,
    email: String,
    age: u32,
    active: bool,
    tags: Vec<String>,
    metadata: Metadata,
}

#[derive(Serialize, JsonSchema)]
struct Metadata {
    created_at: u64,
    updated_at: u64,
    version: u32,
}

impl Endpoint for ComplexJson {
    fn ep(&self) -> Route {
        Route::POST("/api/complex")
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new().summary("Complex nested JSON handling"))
    }
}

#[async_trait]
impl API for ComplexJson {
    type Req = ComplexRequest;
    type Res = Json<ComplexResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        Json(ComplexResponse {
            id: 42,
            name: ctx.req.name,
            email: ctx.req.email,
            age: ctx.req.age,
            active: ctx.req.active,
            tags: ctx.req.tags,
            metadata: Metadata {
                created_at: 1234567890,
                updated_at: 1234567890,
                version: 1,
            },
        })
    }
}

// =============================================================================
// Benchmark 4: Large Payload
// =============================================================================

const LARGE_BODY_1KB: &str = r#"{"data":"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"}"#;

#[derive(Clone)]
struct LargePayload;

#[derive(Default, Deserialize, JsonSchema)]
struct LargeRequest {
    data: String,
}

#[derive(Serialize, JsonSchema)]
struct LargeResponse {
    size: usize,
    echo: String,
}

impl Endpoint for LargePayload {
    fn ep(&self) -> Route {
        Route::POST("/api/large")
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new().summary("Large payload handling (1KB)"))
    }
}

#[async_trait]
impl API for LargePayload {
    type Req = LargeRequest;
    type Res = Json<LargeResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let size = ctx.req.data.len();
        Json(LargeResponse {
            size,
            echo: ctx.req.data,
        })
    }
}

// =============================================================================
// Benchmark 5: Deep Route Matching
// =============================================================================

#[derive(Clone)]
struct DeepRoute;

impl Endpoint for DeepRoute {
    fn ep(&self) -> Route {
        Route::GET("/api/v1/users/123/posts/456/comments/789")
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new().summary("Deep nested route matching"))
    }
}

#[async_trait]
impl API for DeepRoute {
    type Req = ();
    type Res = &'static str;

    async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
        "success"
    }
}

// =============================================================================
// Benchmark Runner
// =============================================================================

fn main() {
    println!("Uncovr Framework Benchmarks\n");
    println!("Running performance tests with rewrk...\n");

    if on_ci() {
        println!("Running on CI - skipping benchmarks");
        return;
    }

    ensure_rewrk_is_installed();

    // Start server with all benchmark endpoints
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let config = AppConfig::new("Benchmark Server", "1.0.0")
                .bind("127.0.0.1:3030")
                .logging(LoggingConfig::disabled());

            uncovr::server::Server::new()
                .with_config(config)
                .register(Ping)
                .register(SimpleJson)
                .register(ComplexJson)
                .register(LargePayload)
                .register(DeepRoute)
                .serve()
                .await
                .expect("Server failed");
        });
    });

    // Give server time to start
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Benchmark Results:\n");
    println!("{}", "=".repeat(80));

    // Run benchmarks
    run_benchmark("1. Minimal Overhead (Ping)", "GET", "/ping", None);
    run_benchmark("2. Simple JSON", "GET", "/api/simple", None);
    run_benchmark(
        "3. Complex JSON",
        "POST",
        "/api/complex",
        Some(
            r#"{"name":"John Doe","email":"john@example.com","age":30,"active":true,"tags":["rust","benchmark"]}"#,
        ),
    );
    run_benchmark(
        "4. Large Payload (1KB)",
        "POST",
        "/api/large",
        Some(LARGE_BODY_1KB),
    );
    run_benchmark(
        "5. Deep Route Matching",
        "GET",
        "/api/v1/users/123/posts/456/comments/789",
        None,
    );

    println!("{}", "=".repeat(80));
    println!("\nBenchmarks completed!");
    println!("\nNote: Server process will exit automatically.");
}

fn run_benchmark(name: &str, method: &str, path: &str, body: Option<&str>) {
    let url = format!("http://127.0.0.1:3030{}", path);

    let mut cmd = Command::new("rewrk");
    cmd.arg("-t")
        .arg("4") // 4 threads
        .arg("-c")
        .arg("100") // 100 connections
        .arg("-d")
        .arg("10s") // 10 second duration
        .arg("-h")
        .arg(&url)
        .arg("--pct"); // Show percentiles

    if method != "GET" {
        cmd.arg("-m").arg(method.to_lowercase());
    }

    if let Some(body_content) = body {
        cmd.arg("-H")
            .arg("content-type: application/json")
            .arg("-b")
            .arg(body_content);
    }

    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    println!("\n{}", name);
    println!("{}", "-".repeat(name.len()));

    let output = cmd.output();

    match output {
        Ok(output) if output.status.success() => {}
        Ok(output) => {
            eprintln!("❌ Benchmark failed with status: {}", output.status);
            eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            eprintln!("❌ Failed to run benchmark: {}", e);
        }
    }
}

fn ensure_rewrk_is_installed() {
    let check = Command::new("rewrk").arg("--version").output();

    if check.is_err() {
        println!("Installing rewrk...");
        install_rewrk();
    }
}

fn install_rewrk() {
    let status = Command::new("cargo")
        .args(["install", "rewrk"])
        .status()
        .expect("Failed to install rewrk");

    if !status.success() {
        panic!("❌ Failed to install rewrk. Please install it manually: cargo install rewrk");
    }

    println!("rewrk installed successfully");
}

fn on_ci() -> bool {
    std::env::var("CI").is_ok() || std::env::var("GITHUB_ACTIONS").is_ok()
}
