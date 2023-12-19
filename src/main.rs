use std::{env, net::SocketAddr, str::FromStr, sync::Arc};
use std::sync::RwLock;
use anyhow::{Result, Ok};
use axum::{Server, Router, routing::{get_service, self, IntoMakeService}};
use mqtt::MqttClient;
use paho_mqtt::Message;
use routes::make_routes;
use sea_orm::Database;
use tokio::{sync::mpsc, net::TcpListener};
use tower_http::services::ServeDir;
use sea_orm::*;
use entity::prelude::*;

mod routes;
mod entity;
mod mqtt;

use entity::mqtt_message;

use crate::entity::mqtt_aht20;

#[tokio::main]
async fn main() -> Result<()> {

    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false);
    let db_connection = Database::connect(opt).await?;
    // println!("connect to db success : {:?}",db_url);
    
    let (s,mut r) = mpsc::channel::<Vec<mqtt_aht20::ActiveModel>>(20);
    let mqtt_url = "tcp://sh.vcxk.fun:1883";
    let client = MqttClient::connect(mqtt_url,s).await?;
    println!("connect to mqtt success : {:?}",mqtt_url);
    let db_connection2 = db_connection.clone();
    tokio::spawn(async move {
        while let Some(msgs) =  r.recv().await{
            if msgs.is_empty() { continue; }
            if let Err(e) = MqttAht20::insert_many(msgs).exec(&db_connection2).await {
                println!("mqtt_ath20 insert many err {:?}",e);
            } else {
                // println!("insert many success");
            }
        }
    });


    let host = env::var("HOST").unwrap_or("0.0.0.0".into());
    let port = env::var("PORT").unwrap_or("8081".into());
    let bind = format!("{}:{}",host,port); 
    let bind = SocketAddr::from_str(&bind).unwrap();
    
    
    let state = Arc::new(AppState{conn:db_connection,mqtt:client});
    let route = make_routes(state);
    // let route = Router::new().with_state(state);
    // route.into_make_service();
    // let listener = TcpListener::bind(&bind).await?;
    println!("to bind server port");
    Server::bind(&bind).serve(route.into_make_service()).await?;
    //     .serve(route.into_make_service()).await?;

    Ok(())
    
}

type SharedState = Arc<AppState>;
#[derive(Clone)]
struct AppState {
    conn:DatabaseConnection,
    mqtt:MqttClient
}