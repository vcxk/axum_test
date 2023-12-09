use std::{time::{Duration, SystemTime}, sync::Arc, ops::Add};

use paho_mqtt::{AsyncClient, Client, ConnectOptions, ConnectOptionsBuilder, QOS_0, Message};
use sea_orm::{Related, ActiveValue, prelude::ChronoDateTime};
use tokio::{time::{sleep, Instant}, sync::mpsc::Sender};


use sea_orm::entity::prelude::*;
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

pub(crate) struct MqttClient {
    client:Arc<Client>
}

fn resolvMessage(msg:&Message) -> mqtt_message::ActiveModel {
    let sn = msg.topic().split("/").last().unwrap().to_string();
    let content = msg.payload_str().to_string();

    let model = mqtt_message::ActiveModel { 
        sn:ActiveValue::Set(sn),
        msg:ActiveValue::Set(content),
        create_time:ActiveValue::Set(Some(chrono::Utc::now().naive_local())),
        ..Default::default() 
    };
    model
}

impl MqttClient {

    pub async fn connect(uri:&str,s:Sender<Vec<mqtt_message::ActiveModel>>) -> anyhow::Result<MqttClient> {

        let cli = Arc::new(Client::new(uri).unwrap());
        let opts = ConnectOptionsBuilder::new_v3()
            .clean_session(true)
            .keep_alive_interval(Duration::from_secs(120))
            .finalize();
        // cli.set_message_callback(mqtt_msg_callback);
        cli.connect(opts)?;
        cli.subscribe("/esp/+/+", QOS_0)?;
        let cli2 = cli.clone();
        tokio::spawn(async move {
            let consuming = cli2.start_consuming();
            let mut last = SystemTime::now();
            let mut temps = Vec::<mqtt_message::ActiveModel>::new();
            loop {
                if SystemTime::now().duration_since(last).unwrap().as_millis() >= 1000 {
                    last = last.add(Duration::from_millis(1000));
                    s.send(temps).await;
                    temps = Vec::<mqtt_message::ActiveModel>::new();
                    //todo!()
                }
                sleep(Duration::from_millis(100)).await;
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
                        let values = payload.split(" ");
                        let values:Vec<&str> = values.collect();
                        if values.len() < 9 { continue; }
                        let hum:f32 = values[5].parse().unwrap();
                        let tem:f32 = values[8].parse().unwrap();
                        println!("hum = {:?} tem = {:?}",hum,tem);
                        temps.push(resolvMessage(&msg));
                    } else {
                        println!("{:?}",topic);
                        println!("{:?}",payload);
                    }
                }
            }
        });
        Ok(MqttClient { client: cli })
    }
}