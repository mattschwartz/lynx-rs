use crate::handlers::{kitty, profile, users};
use axum::{routing::get, routing::post, Extension, Router};
use sqlx::PgPool;

pub fn create_routes(db_pool: PgPool) -> Router {
    Router::new()
        .route("/users", post(users::create_user))
        // deprecated
        .route("/profile", post(profile::create_profile))
        .route("/kitty", post(kitty::add_kitty))
        .route("/kitty", get(kitty::list_kitties))
        .layer(Extension(db_pool))
}
