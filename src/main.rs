use std::{env, net::SocketAddr, str::FromStr};

use anyhow::{Result, Ok};
use axum::{Server, Router, routing::{get_service, self}};
use routes::make_routes;
use sea_orm::Database;
use tower_http::services::ServeDir;

mod routes;

#[tokio::main]
async fn main() -> Result<()> {

    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let host = env::var("HOST").unwrap_or("0.0.0.0".into());
    let port = env::var("PORT").unwrap_or("8081".into());
    let bind = format!("{}:{}",host,port); 
    let bind = SocketAddr::from_str(&bind).unwrap();

    let route = make_routes();
    Server::bind(&bind).serve(route.into_make_service()).await?;

    Ok(())
    
}
