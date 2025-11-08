use uncovr::prelude::*;
use uncovr::server::endpoint::{Docs, Endpoint, Route};

#[derive(Default, Deserialize, JsonSchema)]
pub struct UrlRequest {
    /// The long URL to be shortened
    pub url: String,
}

#[derive(Serialize, JsonSchema)]
pub struct UrlResponse {
    /// The short URL generated for the provided long URL
    pub short_url: String,
}

#[derive(Serialize, JsonSchema)]
pub struct Redirect;

#[derive(Clone)]
pub struct ShortenUrlApi;

#[derive(Clone)]
pub struct RedirectUrlApi;

impl Endpoint for ShortenUrlApi {
    fn ep(&self) -> Route {
        Route::POST("/url")
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Shorten a URL")
                .description("Takes a long URL and returns a shortened version that can be used for redirects")
                .tag("urls")
                .responses(|op| {
                    op.response::<200, Json<UrlResponse>>()
                        .response::<400, Json<ErrorResponse>>()
                        .response::<500, Json<ErrorResponse>>()
                }),
        )
    }
}

impl Endpoint for RedirectUrlApi {
    fn ep(&self) -> Route {
        let mut route = Route::GET("/:id");
        route.path_param("id").desc("The short URL identifier");
        route
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Redirect to original URL")
                .description("Redirects to the original URL associated with the given short URL ID")
                .tag("urls")
                .responses(|op| {
                    op.response::<301, Json<Redirect>>()
                        .response::<404, Json<ErrorResponse>>()
                }),
        )
    }
}
