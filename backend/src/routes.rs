use crate::handlers::profile;
use axum::{routing::post, Extension, Router};
use sqlx::PgPool;

pub fn create_routes(db_pool: PgPool) -> Router {
    Router::new()
        .route("/profile", post(profile::create_profile))
        .layer(Extension(db_pool))
}
