use std::time::Duration;

use axum::{Router, routing::{self, get_service}, extract::{State, Query, Path}, Json};
use paho_mqtt::Message;
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use tokio::time::sleep;
use tower_http::services::ServeDir;
use serde::*;

use crate::{SharedState, entity::mqtt_aht20};
use crate::entity::prelude::*;


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
        // .route("/handle", routing::get(handle))
        .route("/mqtt/set/time/:sn", routing::post(mqtt_set_time))
        .route("/aht20/list", routing::get(aht20_list))
        .with_state(state);
    
    return route;
}

#[derive(Serialize,Deserialize)]
struct MyResponse<T> {
    msg:String,
    code:u64,
    data:Option<T>
}

async fn mqtt_set_time(State(state):State<SharedState>,path:Path<String>) {
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

#[derive(Serialize,Deserialize)]
struct PageInfo {
    page:u64,
    size:u64
}

#[derive(Serialize,Deserialize)]
struct PageResult<T> {
    page:u64,
    size:u64,
    data:Vec<T>,
    total:u64 
}

async fn aht20_list(State(state):State<SharedState>,Query(page): Query<PageInfo>) -> Json<PageResult<mqtt_aht20::Model>>  {
    let db = &state.conn;
    let data = MqttAht20::find()
        .order_by_desc(mqtt_aht20::Column::CreateTime)
        .paginate(db, page.size)
        .fetch_page(page.page)
        .await.unwrap_or(vec![]);
    let count = MqttAht20::find().count(db).await.unwrap_or(0);
    Json(PageResult {
        page:page.page,
        size:page.size,
        data:data,
        total:count
    })
}


