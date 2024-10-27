use axum::{routing::post, Extension, Router};
use sqlx::PgPool;
use crate::handlers::{profile};

pub fn create_routes(db_pool: PgPool) -> Router {
    Router::new()
        .route("/profile", post(profile::create_profile))
        .layer(Extension(db_pool))
}
