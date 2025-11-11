//! OpenAPI Documentation Handlers
//!
//! This module provides handlers for serving OpenAPI documentation in various formats:
//! - JSON specification at `/openapi.json`
//! - Interactive Scalar UI at `/docs`

use aide::openapi::OpenApi;
use axum::extract::Extension;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};

const DOCS_HTML: &str = include_str!("../static/docs.html");

/// Serves the OpenAPI documentation as JSON
///
/// This handler is typically mounted at `/openapi.json` and returns the raw OpenAPI specification.
pub async fn serve_docs(Extension(api): Extension<OpenApi>) -> Response {
    let json = serde_json::to_value(&api).unwrap_or_default();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        json.to_string(),
    )
        .into_response()
}

/// Serves the Scalar UI for API documentation
///
/// This handler is typically mounted at `/docs` and provides an interactive API documentation UI.
/// The `openapi_json_path` parameter specifies the path to fetch the OpenAPI JSON spec.
pub async fn serve_scalar_ui(openapi_json_path: String) -> Response {
    let html = DOCS_HTML.replace("{{OPENAPI_JSON_PATH}}", &openapi_json_path);

    (StatusCode::OK, [(header::CONTENT_TYPE, "text/html")], html).into_response()
}
