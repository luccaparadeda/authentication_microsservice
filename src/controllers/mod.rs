
use axum::{http::StatusCode, response::IntoResponse};


#[utoipa::path(
    get,
    path = "/healthcheck",
    tag = "Healthcheck",
    responses(
        (status = 200, description = "Api está em funcionamento"),
        (status = 400, description = "Bad request")
    )
)]
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK)
}

