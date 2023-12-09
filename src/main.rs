use std::{env, net::SocketAddr, str::FromStr};

use anyhow::{Result, Ok};
use axum::{Server, Router, routing::{get_service, self}};
use mqtt::MqttClient;
use paho_mqtt::Message;
use routes::make_routes;
use sea_orm::Database;
use tokio::sync::mpsc;
use tower_http::services::ServeDir;
use sea_orm::*;
use entity::prelude::*;

mod routes;
mod entity;
mod mqtt;

use entity::mqtt_message;

#[tokio::main]
async fn main() -> Result<()> {

    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    let db_connection = Database::connect(&db_url).await?;
    println!("connect to db success : {:?}",db_url);
    
    let (s,mut r) = mpsc::channel::<Vec<mqtt_message::ActiveModel>>(20);
    let mqtt_url = "tcp://sh.vcxk.fun:1883";
    let client = MqttClient::connect(mqtt_url,s).await?;
    println!("connect to mqtt success : {:?}",mqtt_url);
    tokio::spawn(async move {
        while let Some(msgs) =  r.recv().await{
            if msgs.is_empty() { continue; }
            if let Err(e) = MqttMessage::insert_many(msgs).exec(&db_connection).await {
                println!("mqtt_message insert many err {:?}",e);
            } else {
                println!("insert many success");
            }
        }
    });

    let host = env::var("HOST").unwrap_or("0.0.0.0".into());
    let port = env::var("PORT").unwrap_or("8081".into());
    let bind = format!("{}:{}",host,port); 
    let bind = SocketAddr::from_str(&bind).unwrap();
    
    let route = make_routes();
    println!("to bind server port");
    Server::bind(&bind).serve(route.into_make_service()).await?;

    Ok(())
    
}
