use axum::{extract::Json, response::IntoResponse, Extension};
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct AddKittyRequest {
    name: String,
}

pub async fn add_kitty(
    Extension(db_pool): Extension<PgPool>,
    Json(payload): Json<AddKittyRequest>,
) -> impl IntoResponse {
    if payload.name.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Name cannot be empty".to_string()));
    }

    Ok((StatusCode::OK, Json("")))
}

pub async fn list_kitties() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(format!("Here's ur kitties: {}", "bill".to_string())),
    )
}
