use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};
use molly::{AWS_IOT_MQTT_ALPN, HELLO_WORLD_TOPIC, KEEP_ALIVE_INTERVAL};
use rumqttc::{AsyncClient, MqttOptions, QoS, TlsConfiguration, Transport};
use serde_json::json;
use std::fs;
use std::{error::Error, time::Duration};
use tokio::{task, time};


const CLIENT_ID: &str = "molly";

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

/// The current_thread runtime flavor is a lightweight, single-threaded runtime. 
/// It is a good choice when only spawning a few tasks and opening a handful of sockets. 
/// https://tokio.rs/tokio/tutorial/shared-state
#[tokio::main(flavor = "current_thread")]

async fn main() -> Result<(), Box<dyn Error>> {
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("received ctrl-c event");
        // disconnect mqtt client and exit application.
        std::process::exit(0);
    });

    env_logger::init();

    molly::Configs::init();

    let name = CLIENT_ID;
    let endpoint = molly::Configs::global().iot_ats.to_string();
    // let ca = molly::Configs::global().ca.to_string();
    // let cert = molly::Configs::global().cert.to_string();
    // let key = molly::Configs::global().key.to_string();

    info!("{name} is connecting to {endpoint}");
    let mut mqttoptions = MqttOptions::new(name, endpoint, 8883);
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(KEEP_ALIVE_INTERVAL));

    let aws_ca = include_bytes!("../certs/AmazonRootCA1.pem");
    let client_cert = include_bytes!("../certs/certificate.pem.crt");
    let client_key = include_bytes!("../certs/private.pem.key");


    let transport = Transport::Tls(TlsConfiguration::Simple {
        ca: aws_ca.to_vec(),
        alpn: None,
        client_auth: Some((client_cert.to_vec(), client_key.to_vec())),
    });

    mqttoptions.set_transport(transport);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client
    .subscribe(HELLO_WORLD_TOPIC, QoS::AtMostOnce)
    .await
    .unwrap();

    task::spawn(async move {
        let full_name = "John Doe";
let age_last_year = 42;

// The type of `john` is `serde_json::Value`
let john = json!({
    "name": full_name,
    "age": age_last_year + 1,
    "phones": [
        format!("+86 {}", 1234567890)
    ]
});
        for i in 0..10 {
            client.publish(HELLO_WORLD_TOPIC, QoS::AtLeastOnce, false, john.to_string()).await.unwrap();
            time::sleep(Duration::from_millis(100)).await;
        }
    });

    while let Ok(notification) = eventloop.poll().await {
        println!("Received = {:?}", notification);
    }

    Ok(())
}


#[test]
fn find_a_match() {
    let mut result = Vec::new();
    molly::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

