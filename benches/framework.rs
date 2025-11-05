#![allow(missing_docs)]

use serde::{Deserialize, Serialize};
use std::{
    future::IntoFuture,
    io::BufRead,
    process::{Command, Stdio},
};
use uncovr::prelude::*;
use uncovr::server::Server;

fn main() {
    if on_ci() {
        install_rewrk();
    } else {
        ensure_rewrk_is_installed();
    }

    // Minimal endpoint - no JSON, just raw response
    benchmark("minimal-ping").run(|| {
        #[derive(Clone)]
        struct MinimalPing;

        impl Metadata for MinimalPing {
            fn metadata(&self) -> Endpoint {
                Endpoint::new("/", "get")
            }
        }

        #[async_trait]
        impl API for MinimalPing {
            type Req = ();
            type Res = &'static str;

            async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
                "pong"
            }
        }

        Server::new()
            .with_config(minimal_config())
            .register(MinimalPing)
            .build()
    });

    // Basic endpoint with simple JSON response
    benchmark("basic-json").run(|| {
        #[derive(Clone)]
        struct BasicJson;

        #[derive(Serialize, JsonSchema)]
        struct SimpleResponse {
            message: String,
        }

        impl Metadata for BasicJson {
            fn metadata(&self) -> Endpoint {
                Endpoint::new("/", "get").summary("Basic JSON endpoint")
            }
        }

        #[async_trait]
        impl API for BasicJson {
            type Req = ();
            type Res = Json<SimpleResponse>;

            async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
                Json(SimpleResponse {
                    message: "Hello, World!".to_string(),
                })
            }
        }

        Server::new()
            .with_config(minimal_config())
            .register(BasicJson)
            .build()
    });

    // Receive JSON - POST with JSON body
    benchmark("receive-json")
        .method("post")
        .headers(&[("content-type", "application/json")])
        .body(r#"{"name": "John", "value": 123}"#)
        .run(|| {
            #[derive(Clone)]
            struct ReceiveJson;

            #[derive(Default, Deserialize, JsonSchema)]
            struct InputData {
                #[allow(dead_code)]
                name: String,
                #[allow(dead_code)]
                value: u32,
            }

            impl Metadata for ReceiveJson {
                fn metadata(&self) -> Endpoint {
                    Endpoint::new("/", "post").summary("Receive JSON data")
                }
            }

            #[async_trait]
            impl API for ReceiveJson {
                type Req = InputData;
                type Res = &'static str;

                async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
                    "ok"
                }
            }

            Server::new()
                .with_config(minimal_config())
                .register(ReceiveJson)
                .build()
        });

    // Send and receive JSON
    benchmark("echo-json")
        .method("post")
        .headers(&[("content-type", "application/json")])
        .body(r#"{"text": "hello world"}"#)
        .run(|| {
            #[derive(Clone)]
            struct EchoJson;

            #[derive(Default, Deserialize, JsonSchema)]
            struct EchoRequest {
                text: String,
            }

            #[derive(Serialize, JsonSchema)]
            struct EchoResponse {
                echo: String,
            }

            impl Metadata for EchoJson {
                fn metadata(&self) -> Endpoint {
                    Endpoint::new("/", "post").summary("Echo JSON")
                }
            }

            #[async_trait]
            impl API for EchoJson {
                type Req = EchoRequest;
                type Res = Json<EchoResponse>;

                async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
                    Json(EchoResponse { echo: ctx.req.text })
                }
            }

            Server::new()
                .with_config(minimal_config())
                .register(EchoJson)
                .build()
        });

    // Complex JSON payload
    benchmark("complex-json")
        .method("post")
        .headers(&[("content-type", "application/json")])
        .body(r#"{"name": "John Doe", "email": "john@example.com", "age": 30, "active": true, "tags": ["user", "premium"]}"#)
        .run(|| {
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
            }

            impl Metadata for ComplexJson {
                fn metadata(&self) -> Endpoint {
                    Endpoint::new("/", "post")
                        .summary("Complex JSON handling")
                }
            }

            #[async_trait]
            impl API for ComplexJson {
                type Req = ComplexRequest;
                type Res = Json<ComplexResponse>;

                async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
                    Json(ComplexResponse {
                        id: 1,
                        name: ctx.req.name,
                        email: ctx.req.email,
                        age: ctx.req.age,
                        active: ctx.req.active,
                        tags: ctx.req.tags,
                    })
                }
            }

            Server::new()
                .with_config(minimal_config())
                .register(ComplexJson)
                .build()
        });

    // Multiple endpoints registered
    benchmark("multiple-endpoints")
        .path("/users")
        .method("post")
        .headers(&[("content-type", "application/json")])
        .body(r#"{"name": "John", "email": "john@example.com"}"#)
        .run(|| {
            #[derive(Clone)]
            struct Ping;

            impl Metadata for Ping {
                fn metadata(&self) -> Endpoint {
                    Endpoint::new("/ping", "get")
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

            #[derive(Clone)]
            struct Users;

            #[derive(Default, Deserialize, JsonSchema)]
            struct CreateUser {
                name: String,
                email: String,
            }

            #[derive(Serialize, JsonSchema)]
            struct User {
                id: u64,
                name: String,
                email: String,
            }

            impl Metadata for Users {
                fn metadata(&self) -> Endpoint {
                    Endpoint::new("/users", "post").summary("Create user")
                }
            }

            #[async_trait]
            impl API for Users {
                type Req = CreateUser;
                type Res = Json<User>;

                async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
                    Json(User {
                        id: 1,
                        name: ctx.req.name,
                        email: ctx.req.email,
                    })
                }
            }

            #[derive(Clone)]
            struct Health;

            impl Metadata for Health {
                fn metadata(&self) -> Endpoint {
                    Endpoint::new("/health", "get")
                }
            }

            #[async_trait]
            impl API for Health {
                type Req = ();
                type Res = &'static str;

                async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
                    "ok"
                }
            }

            Server::new()
                .with_config(minimal_config())
                .register(Ping)
                .register(Users)
                .register(Health)
                .build()
        });

    // Full config with CORS and logging
    benchmark("with-cors-logging").run(|| {
        #[derive(Clone)]
        struct FullConfigPing;

        impl Metadata for FullConfigPing {
            fn metadata(&self) -> Endpoint {
                Endpoint::new("/", "get")
            }
        }

        #[async_trait]
        impl API for FullConfigPing {
            type Req = ();
            type Res = &'static str;

            async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
                "pong"
            }
        }

        let config = AppConfig::new("Benchmark API", "1.0.0")
            .logging(LoggingConfig::production())
            .cors(CorsConfig::development())
            .environment(Environment::Development)
            .docs(false);

        Server::new()
            .with_config(config)
            .register(FullConfigPing)
            .build()
    });

    // PUT request
    benchmark("http-put")
        .method("put")
        .headers(&[("content-type", "application/json")])
        .body(r#"{"id": 1, "name": "Updated"}"#)
        .run(|| {
            #[derive(Clone)]
            struct UpdateResource;

            #[derive(Default, Deserialize, JsonSchema)]
            struct UpdateRequest {
                #[allow(dead_code)]
                id: u64,
                #[allow(dead_code)]
                name: String,
            }

            impl Metadata for UpdateResource {
                fn metadata(&self) -> Endpoint {
                    Endpoint::new("/", "put").summary("Update resource")
                }
            }

            #[async_trait]
            impl API for UpdateResource {
                type Req = UpdateRequest;
                type Res = &'static str;

                async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
                    "updated"
                }
            }

            Server::new()
                .with_config(minimal_config())
                .register(UpdateResource)
                .build()
        });

    // PATCH request
    benchmark("http-patch")
        .method("patch")
        .headers(&[("content-type", "application/json")])
        .body(r#"{"name": "Patched"}"#)
        .run(|| {
            #[derive(Clone)]
            struct PatchResource;

            #[derive(Default, Deserialize, JsonSchema)]
            struct PatchRequest {
                #[allow(dead_code)]
                name: String,
            }

            impl Metadata for PatchResource {
                fn metadata(&self) -> Endpoint {
                    Endpoint::new("/", "patch").summary("Patch resource")
                }
            }

            #[async_trait]
            impl API for PatchResource {
                type Req = PatchRequest;
                type Res = &'static str;

                async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
                    "patched"
                }
            }

            Server::new()
                .with_config(minimal_config())
                .register(PatchResource)
                .build()
        });

    // DELETE request
    benchmark("http-delete")
        .method("delete")
        .headers(&[("content-type", "application/json")])
        .body(r#"{"id": 1}"#)
        .run(|| {
            #[derive(Clone)]
            struct DeleteResource;

            #[derive(Default, Deserialize, JsonSchema)]
            struct DeleteRequest {
                #[allow(dead_code)]
                id: u64,
            }

            impl Metadata for DeleteResource {
                fn metadata(&self) -> Endpoint {
                    Endpoint::new("/", "delete").summary("Delete resource")
                }
            }

            #[async_trait]
            impl API for DeleteResource {
                type Req = DeleteRequest;
                type Res = &'static str;

                async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
                    "deleted"
                }
            }

            Server::new()
                .with_config(minimal_config())
                .register(DeleteResource)
                .build()
        });

    // Large JSON payload (1KB)
    const LARGE_BODY_1KB: &str = r#"{"data": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"}"#;
    benchmark("large-json-1kb")
        .method("post")
        .headers(&[("content-type", "application/json")])
        .body(LARGE_BODY_1KB)
        .run(|| {
            #[derive(Clone)]
            struct LargePayload;

            #[derive(Default, Deserialize, JsonSchema)]
            struct LargeRequest {
                #[allow(dead_code)]
                data: String,
            }

            #[derive(Serialize, JsonSchema)]
            struct LargeResponse {
                size: usize,
            }

            impl Metadata for LargePayload {
                fn metadata(&self) -> Endpoint {
                    Endpoint::new("/", "post").summary("Handle large payload")
                }
            }

            #[async_trait]
            impl API for LargePayload {
                type Req = LargeRequest;
                type Res = Json<LargeResponse>;

                async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
                    Json(LargeResponse {
                        size: ctx.req.data.len(),
                    })
                }
            }

            Server::new()
                .with_config(minimal_config())
                .register(LargePayload)
                .build()
        });

    // Array of objects
    benchmark("json-array")
        .method("post")
        .headers(&[("content-type", "application/json")])
        .body(r#"{"items": [{"id": 1, "name": "Item 1"}, {"id": 2, "name": "Item 2"}, {"id": 3, "name": "Item 3"}]}"#)
        .run(|| {
            #[derive(Clone)]
            struct ArrayEndpoint;

            #[derive(Default, Deserialize, JsonSchema)]
            struct Item {
                #[allow(dead_code)]
                id: u64,
                #[allow(dead_code)]
                name: String,
            }

            #[derive(Default, Deserialize, JsonSchema)]
            struct ArrayRequest {
                items: Vec<Item>,
            }

            #[derive(Serialize, JsonSchema)]
            struct ArrayResponse {
                count: usize,
            }

            impl Metadata for ArrayEndpoint {
                fn metadata(&self) -> Endpoint {
                    Endpoint::new("/", "post").summary("Handle array of items")
                }
            }

            #[async_trait]
            impl API for ArrayEndpoint {
                type Req = ArrayRequest;
                type Res = Json<ArrayResponse>;

                async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
                    Json(ArrayResponse {
                        count: ctx.req.items.len(),
                    })
                }
            }

            Server::new()
                .with_config(minimal_config())
                .register(ArrayEndpoint)
                .build()
        });

    // Nested JSON structure
    benchmark("nested-json")
        .method("post")
        .headers(&[("content-type", "application/json")])
        .body(r#"{"user": {"profile": {"name": "John", "email": "john@example.com"}, "settings": {"theme": "dark", "notifications": true}}}"#)
        .run(|| {
            #[derive(Clone)]
            struct NestedEndpoint;

            #[derive(Default, Deserialize, JsonSchema)]
            struct Profile {
                #[allow(dead_code)]
                name: String,
                #[allow(dead_code)]
                email: String,
            }

            #[derive(Default, Deserialize, JsonSchema)]
            struct Settings {
                #[allow(dead_code)]
                theme: String,
                #[allow(dead_code)]
                notifications: bool,
            }

            #[derive(Default, Deserialize, JsonSchema)]
            struct UserData {
                #[allow(dead_code)]
                profile: Profile,
                #[allow(dead_code)]
                settings: Settings,
            }

            #[derive(Default, Deserialize, JsonSchema)]
            struct NestedRequest {
                #[allow(dead_code)]
                user: UserData,
            }

            impl Metadata for NestedEndpoint {
                fn metadata(&self) -> Endpoint {
                    Endpoint::new("/", "post").summary("Handle nested JSON")
                }
            }

            #[async_trait]
            impl API for NestedEndpoint {
                type Req = NestedRequest;
                type Res = &'static str;

                async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
                    "ok"
                }
            }

            Server::new()
                .with_config(minimal_config())
                .register(NestedEndpoint)
                .build()
        });

    // With OpenAPI documentation enabled
    benchmark("with-openapi-docs").run(|| {
        #[derive(Clone)]
        struct DocsEndpoint;

        impl Metadata for DocsEndpoint {
            fn metadata(&self) -> Endpoint {
                Endpoint::new("/", "get").summary("Endpoint with OpenAPI docs")
            }
        }

        #[async_trait]
        impl API for DocsEndpoint {
            type Req = ();
            type Res = &'static str;

            async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
                "pong"
            }
        }

        let config = AppConfig::new("Benchmark API", "1.0.0")
            .environment(Environment::Development)
            .docs(true);

        Server::new()
            .with_config(config)
            .register(DocsEndpoint)
            .build()
    });

    // Deep routing - many levels
    benchmark("deep-routing")
        .path("/api/v1/users/123/posts/456/comments")
        .method("post")
        .headers(&[("content-type", "application/json")])
        .body(r#"{"text": "comment"}"#)
        .run(|| {
            #[derive(Clone)]
            struct DeepRoute;

            #[derive(Default, Deserialize, JsonSchema)]
            struct CommentRequest {
                #[allow(dead_code)]
                text: String,
            }

            impl Metadata for DeepRoute {
                fn metadata(&self) -> Endpoint {
                    Endpoint::new("/api/v1/users/123/posts/456/comments", "post")
                        .summary("Deep nested route")
                }
            }

            #[async_trait]
            impl API for DeepRoute {
                type Req = CommentRequest;
                type Res = &'static str;

                async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
                    "created"
                }
            }

            Server::new()
                .with_config(minimal_config())
                .register(DeepRoute)
                .build()
        });

    // Many endpoints
    benchmark("many-endpoints")
        .path("/api/users")
        .method("post")
        .headers(&[("content-type", "application/json")])
        .body(r#"{"name": "John"}"#)
        .run(|| {
            // Create many endpoint types
            macro_rules! create_endpoint {
                ($name:ident, $path:expr, $method:expr) => {
                    #[derive(Clone)]
                    struct $name;

                    impl Metadata for $name {
                        fn metadata(&self) -> Endpoint {
                            Endpoint::new($path, $method)
                        }
                    }

                    #[async_trait]
                    impl API for $name {
                        type Req = SimpleReq;
                        type Res = &'static str;

                        async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
                            "ok"
                        }
                    }
                };
            }

            #[derive(Default, Deserialize, JsonSchema)]
            struct SimpleReq {
                #[allow(dead_code)]
                name: String,
            }

            create_endpoint!(GetUsers, "/api/users", "get");
            create_endpoint!(CreateUser, "/api/users", "post");
            create_endpoint!(GetPosts, "/api/posts", "get");
            create_endpoint!(CreatePost, "/api/posts", "post");
            create_endpoint!(GetComments, "/api/comments", "get");
            create_endpoint!(CreateComment, "/api/comments", "post");
            create_endpoint!(GetProducts, "/api/products", "get");
            create_endpoint!(CreateProduct, "/api/products", "post");
            create_endpoint!(GetOrders, "/api/orders", "get");
            create_endpoint!(CreateOrder, "/api/orders", "post");
            create_endpoint!(GetProfile, "/api/profile", "get");
            create_endpoint!(UpdateProfile, "/api/profile", "put");
            create_endpoint!(GetSettings, "/api/settings", "get");
            create_endpoint!(UpdateSettings, "/api/settings", "put");
            create_endpoint!(GetNotifications, "/api/notifications", "get");
            create_endpoint!(DeleteNotification, "/api/notifications", "delete");
            create_endpoint!(GetAnalytics, "/api/analytics", "get");
            create_endpoint!(GetReports, "/api/reports", "get");
            create_endpoint!(GetDashboard, "/api/dashboard", "get");
            create_endpoint!(GetHealth, "/api/health", "get");

            Server::new()
                .with_config(minimal_config())
                .register(GetUsers)
                .register(CreateUser)
                .register(GetPosts)
                .register(CreatePost)
                .register(GetComments)
                .register(CreateComment)
                .register(GetProducts)
                .register(CreateProduct)
                .register(GetOrders)
                .register(CreateOrder)
                .register(GetProfile)
                .register(UpdateProfile)
                .register(GetSettings)
                .register(UpdateSettings)
                .register(GetNotifications)
                .register(DeleteNotification)
                .register(GetAnalytics)
                .register(GetReports)
                .register(GetDashboard)
                .register(GetHealth)
                .build()
        });
}

fn minimal_config() -> AppConfig {
    AppConfig::new("Benchmark", "1.0.0")
        .environment(Environment::Development)
        .docs(false)
}

fn benchmark(name: &'static str) -> BenchmarkBuilder {
    BenchmarkBuilder {
        name,
        path: None,
        method: None,
        headers: None,
        body: None,
    }
}

struct BenchmarkBuilder {
    name: &'static str,
    path: Option<&'static str>,
    method: Option<&'static str>,
    headers: Option<&'static [(&'static str, &'static str)]>,
    body: Option<&'static str>,
}

macro_rules! config_method {
    ($name:ident, $ty:ty) => {
        fn $name(mut self, $name: $ty) -> Self {
            self.$name = Some($name);
            self
        }
    };
}

impl BenchmarkBuilder {
    config_method!(path, &'static str);
    config_method!(method, &'static str);
    config_method!(headers, &'static [(&'static str, &'static str)]);
    config_method!(body, &'static str);

    fn run<F>(self, f: F)
    where
        F: FnOnce() -> uncovr::server::Server,
    {
        // support only running some benchmarks with
        // ```
        // cargo bench -- echo-json complex-json
        // ```
        let args = std::env::args().collect::<Vec<_>>();
        if args.len() != 1 {
            let names = &args[1..args.len() - 1];
            if !names.is_empty() && !names.contains(&self.name.to_owned()) {
                return;
            }
        }

        // Get the server which contains the router
        let server = f();
        let app = server.into_router().into_make_service();

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let listener = rt
            .block_on(tokio::net::TcpListener::bind("0.0.0.0:0"))
            .unwrap();
        let addr = listener.local_addr().unwrap();

        std::thread::spawn(move || {
            rt.block_on(axum::serve(listener, app).into_future())
                .unwrap();
        });

        let mut cmd = Command::new("rewrk");
        cmd.stdout(Stdio::piped());

        cmd.arg("--host");
        cmd.arg(format!("http://{addr}{}", self.path.unwrap_or("")));

        cmd.args(["--connections", "10"]);
        cmd.args(["--threads", "10"]);

        if on_ci() {
            // don't slow down CI by running the benchmarks for too long
            // but do run them for a bit
            cmd.args(["--duration", "1s"]);
        } else {
            cmd.args(["--duration", "10s"]);
        }

        if let Some(method) = self.method {
            cmd.args(["--method", method]);
        }

        for (key, value) in self.headers.into_iter().flatten() {
            cmd.arg("--header");
            cmd.arg(format!("{key}: {value}"));
        }

        if let Some(body) = self.body {
            cmd.args(["--body", body]);
        }

        eprintln!("Running {:?} benchmark", self.name);

        // indent output from `rewrk` so it's easier to read when running multiple benchmarks
        let mut child = cmd.spawn().unwrap();
        let stdout = child.stdout.take().unwrap();
        let stdout = std::io::BufReader::new(stdout);
        for line in stdout.lines() {
            let line = line.unwrap();
            println!("  {line}");
        }

        let status = child.wait().unwrap();

        if !status.success() {
            eprintln!("`rewrk` command failed");
            std::process::exit(status.code().unwrap());
        }
    }
}

fn install_rewrk() {
    println!("installing rewrk");
    let mut cmd = Command::new("cargo");
    cmd.args([
        "install",
        "rewrk",
        "--git",
        "https://github.com/ChillFish8/rewrk.git",
    ]);
    let status = cmd
        .status()
        .unwrap_or_else(|_| panic!("failed to install rewrk"));
    if !status.success() {
        panic!("failed to install rewrk");
    }
}

fn ensure_rewrk_is_installed() {
    let mut cmd = Command::new("rewrk");
    cmd.arg("--help");
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());
    cmd.status().unwrap_or_else(|_| {
        panic!("rewrk is not installed. See https://github.com/lnx-search/rewrk")
    });
}

fn on_ci() -> bool {
    std::env::var("GITHUB_ACTIONS").is_ok()
}
