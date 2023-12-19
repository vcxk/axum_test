use std::{time::{Duration, SystemTime}, sync::Arc, ops::Add, thread::panicking};

use paho_mqtt::{AsyncClient, Client, ConnectOptions, ConnectOptionsBuilder, QOS_0, Message};
use sea_orm::{Related, ActiveValue, prelude::ChronoDateTime};
use tokio::{time::{sleep, Instant}, sync::mpsc::{Sender, self}};


use sea_orm::entity::prelude::*;
use crate::entity::mqtt_aht20;

use super::entity::mqtt_message;

fn mqtt_msg_callback(cli:&AsyncClient,msg:Option<Message>) {
    if msg.is_none() { return; }
    let msg = msg.unwrap();
    let topic = msg.topic();
    let payload = msg.payload_str();
    println!("{:?}",topic);
    println!("{:?}",payload)
}

pub(crate) async fn start_mqtt_client_loop() -> anyhow::Result<()> {
    
    let cli = AsyncClient::new("tcp://sh.vcxk.fun:1883").unwrap();
    let opts = ConnectOptionsBuilder::new_v3()
        .automatic_reconnect(Duration::from_secs(30), Duration::from_secs(120))
        .clean_session(true)
        .keep_alive_interval(Duration::from_secs(120))
        .finalize();
    cli.set_message_callback(mqtt_msg_callback);
    cli.connect(opts).await?;
    cli.subscribe("/esp/+/+", QOS_0).await?;
    sleep(Duration::from_secs(100)).await;
    Ok(())
    
}

#[derive(Clone)]
pub(crate) struct MqttClient {
    pub client:Arc<AsyncClient>,
    // sender:Sender<Message>
}

fn resolve_ath20_msg(msg: &Message) -> Option<mqtt_aht20::ActiveModel> {
    let true = msg.topic().contains("/aht20") else { return None; };
    let Some(sn) = msg.topic().split("/").last() else { return None; };
    let payload = msg.payload_str();
    let values = payload.split(" ");
    let values:Vec<&str> = values.collect();
    if values.len() < 9 { return None; }
    let Ok(hum) = values[5].parse() else { return None; };
    let Ok(tem) = values[8].parse() else { return None; };
    Some(mqtt_aht20::ActiveModel {
        sn: ActiveValue::Set(sn.to_string()),
        humidity: ActiveValue::Set(hum),
        temperature: ActiveValue::Set(tem),
        create_time: ActiveValue::Set(chrono::Utc::now().naive_local()),
        ..Default::default()
    })
}

impl MqttClient {

    pub async fn connect(uri:&str,s:Sender<Vec<mqtt_aht20::ActiveModel>>) -> anyhow::Result<MqttClient> {

        let cli = Arc::new(AsyncClient::new(uri).unwrap());
        let opts = ConnectOptionsBuilder::new_v3()
            .clean_session(true)
            .keep_alive_interval(Duration::from_secs(120))
            .finalize();
        // cli.set_message_callback(mqtt_msg_callback);
        cli.connect(opts).await?;
        cli.subscribe("/esp/+/+", QOS_0).await?;
        let cli2 = cli.clone();
        tokio::spawn(async move {
            let consuming = cli2.start_consuming();
            let mut last = SystemTime::now();

            let mut ath20s = Vec::<mqtt_aht20::ActiveModel>::new();

            loop {
                if SystemTime::now().duration_since(last).unwrap().as_millis() >= 1000 {
                    last = last.add(Duration::from_millis(1000));
                    s.send(ath20s).await;
                    ath20s = Vec::<mqtt_aht20::ActiveModel>::new();
                    //todo!()
                } else {
                    sleep(Duration::from_millis(100)).await;
                }
                
                while !consuming.is_empty() {
                    let msg = match consuming.try_recv() {
                        Err(e) => break,
                        Ok(o) => o
                    };
                    let msg = match msg {
                        None => break,
                        Some(msg) => msg 
                    };
    
                    let topic = msg.topic();
                    let payload = msg.payload_str();
                    
    
                    if topic.contains("/aht20") {
                        if let Some(ath20) = resolve_ath20_msg(&msg) {
                            ath20s.push(ath20);
                        } else {
                            println!("get aht20 model fail");
                        }
                    }
                    println!("{:?} : {:?}",topic,payload);
                    
                }
            }
            println!("mqtt loop out");
        });

        // let (s,mut c) = mpsc::channel(128);
        // let ccli = cli.clone();
        // tokio::spawn(async move {
        //     loop {
        //         if let Some(msg) = c.recv().await {
        //             ccli.publish(msg);
        //         };
        //     }
        // });
        Ok(MqttClient { 
            client: cli, 
            // sender:s
        } )
    }
}