use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().ok(); // Load environment variables from `.env` file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a new connection pool
    PgPool::connect(&database_url).await
}
