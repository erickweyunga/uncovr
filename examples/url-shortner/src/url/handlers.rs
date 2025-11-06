use crate::{
    fun,
    url::apis::{UrlApi, UrlRequest, UrlResponse},
};
use uncovr::prelude::*;

#[async_trait]
impl API for UrlApi {
    type Req = UrlRequest;
    type Res = ApiResponse<UrlResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        // Validate URL is not empty
        if ctx.req.url.is_empty() {
            return ApiResponse::BadRequest("URL cannot be empty");
        }

        // Validate URL format
        if !ctx.req.url.starts_with("http://") && !ctx.req.url.starts_with("https://") {
            return ApiResponse::BadRequest("URL must start with http:// or https://");
        }

        let base_url = "http://localhost:8080";
        let original_url = ctx.req.url.clone();

        // Shorten the URL
        let short_url = fun::shorten_url(&original_url, base_url);

        ApiResponse::Ok(UrlResponse { short_url })
    }
}
