use axum::{extract::Json, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CreateProfileRequest {
    username: String,
    email: String,
}

pub async fn create_profile(
    Extension(db_pool): Extension<PgPool>,
    Json(payload): Json<CreateProfileRequest>,
) -> impl IntoResponse {
    if payload.username.is_empty() || payload.email.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username and email cannot be empty".to_string(),
        ));
    }

    let query = "INSERT INTO profiles (username, email) VALUES ($1, $2)";
    match sqlx::query(query)
        .bind(&payload.username)
        .bind(&payload.email)
        .execute(&db_pool)
        .await
    {
        Ok(_) => Ok(Json(format!(
            "Profile {} created successfully",
            payload.username
        ))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create profile: {}", e),
        )),
    }
}
