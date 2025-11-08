use uncovr::server::endpoint::{Docs, Endpoint, Route};
use uncovr::{prelude::*, server::Server};

// path parameter
#[derive(Clone)]
pub struct GreetByName;

impl Endpoint for GreetByName {
    fn ep(&self) -> Route {
        let mut route = Route::GET("/greet/:name");
        route.path_param("name").desc("Name of the person to greet");
        route
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Greet someone by name")
                .description(
                    "Returns a personalized greeting using the name from the path parameter",
                )
                .tag("greetings"),
        )
    }
}

#[async_trait]
impl API for GreetByName {
    type Req = ();
    type Res = String;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let name = ctx.path.get("name").unwrap_or("stranger");
        format!("Hello, {}!", name)
    }
}

// query parameters
#[derive(Clone)]
pub struct ListItems;

impl Endpoint for ListItems {
    fn ep(&self) -> Route {
        let mut route = Route::GET("/items");
        route.query("page").desc("Page number");
        route.query("limit").desc("Items per page");
        route
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("List items with pagination")
                .description("Returns a paginated list of items. Use page and limit query parameters to control pagination.")
                .tag("items"),
        )
    }
}

#[async_trait]
impl API for ListItems {
    type Req = ();
    type Res = String;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let page = ctx.query.get_u32("page").unwrap_or(1);
        let limit = ctx.query.get_u32("limit").unwrap_or(10);
        format!("Page {} with {} items per page", page, limit)
    }
}

// multiple path params
#[derive(Clone)]
pub struct GetUserPost;

impl Endpoint for GetUserPost {
    fn ep(&self) -> Route {
        let mut route = Route::GET("/users/:user_id/posts/:post_id");
        route.path_param("user_id").desc("User ID");
        route.path_param("post_id").desc("Post ID");
        route
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Get a specific post from a user")
                .description("Retrieves a specific post by ID that belongs to a specific user")
                .tag("users")
                .tag("posts"),
        )
    }
}

#[async_trait]
impl API for GetUserPost {
    type Req = ();
    type Res = String;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let user_id = ctx.path.get_u64("user_id").unwrap_or(0);
        let post_id = ctx.path.get_u64("post_id").unwrap_or(0);
        format!("User {} - Post {}", user_id, post_id)
    }
}

#[tokio::main]
async fn main() {
    let config = AppConfig::new("Routes Example", "0.1.1")
        .bind("127.0.0.1:8000")
        .environment(Environment::Development);

    Server::new()
        .with_config(config)
        .register(GreetByName)
        .register(ListItems)
        .register(GetUserPost)
        .serve()
        .await
        .expect("Server Failed")
}
