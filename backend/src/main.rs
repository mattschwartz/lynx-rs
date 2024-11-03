use std::net::SocketAddr;
use dotenv::dotenv;

mod handlers;
mod db;
mod auth;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_pool = db::init_pool().await.unwrap();

    let app = routes::create_routes(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Backend server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
