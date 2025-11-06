use uncovr::prelude::*;

#[derive(Default, Deserialize, JsonSchema)]
pub struct UrlRequest {
    pub url: String,
}

#[derive(Serialize, JsonSchema)]
pub struct UrlResponse {
    pub short_url: String,
}

#[derive(Clone)]
pub struct UrlApi;

impl Metadata for UrlApi {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/url", "post")
            .summary("Shorten a URL")
            .description("Takes a long URL and returns a shortened version")
    }
}
