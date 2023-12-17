use std::time::Duration;

use axum::{Router, routing::{self, get_service}, extract::{State, Query, Path}};
use paho_mqtt::Message;
use tokio::time::sleep;
use tower_http::services::ServeDir;
use serde::*;

use crate::SharedState;


pub fn make_routes(state:SharedState) -> Router {
    let static_dir = concat!(env!("CARGO_MANIFEST_DIR"),"/static");
    let route = Router::new()
        .route("/hello", routing::get("hello world"))
        .nest_service(
            "/static", 
            get_service(
                ServeDir::new(static_dir)
            )
        )
        .route("/handle", routing::get(handle))
        .route("/mqtt/set/time/:sn", routing::get(handle2))
        .with_state(state);
    
    return route;
}

#[derive(Serialize,Deserialize)]
struct MyResponse<T> {
    msg:String,
    code:u64,
    data:Option<T>
}

async fn handle(State(state):State<SharedState>) -> String {
    "ok".to_string()
}

async fn handle2(State(state):State<SharedState>,path:Path<String>) {
    let mqtt = &state.mqtt;
    println!("path = {:?}",path.0);
    let now = chrono::Utc::now();
    let ts = now.timestamp_millis();
    let token = mqtt.client.publish(Message::new(format!("/esp/act/{}",path.0), format!("time:90={:?}\n",ts), 1));
    
    if let Err(e) = token.await {
        println!("mqtt client publish fail {:?}",e);
    };
    println!("set time to {:?}",ts);
    // sleep(Duration::from_secs(1)).await
}


