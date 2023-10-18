use axum::{Router, routing::{self, get_service}};
use tower_http::services::ServeDir;


pub fn make_routes() -> Router {
    let static_dir = concat!(env!("CARGO_MANIFEST_DIR"),"/static");
    let route = Router::new()
        .route("/hello", routing::get("hello world"))
        .nest_service(
            "/static", 
            get_service(
                ServeDir::new(static_dir)
            )
        );
    return route;
}

