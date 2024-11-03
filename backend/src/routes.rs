use crate::handlers::users;
use axum::{routing::post, Extension, Router};
use sqlx::PgPool;

pub fn create_routes(db_pool: PgPool) -> Router {
    Router::new()
        .route("/users", post(users::create_user))
        .layer(Extension(db_pool))
}
