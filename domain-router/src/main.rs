use axum::{Router, middleware, routing::get};

use std::net::SocketAddr;
use tower_http::services::ServeDir;

use domain_router::handlers::{dump, location_page};
use domain_router::middleware::context_middleware;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(dump))
        .route("/location", get(location_page))
        .nest_service("/pkg", ServeDir::new("pkg"))
        .layer(middleware::from_fn(context_middleware));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
