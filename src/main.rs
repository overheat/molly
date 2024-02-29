use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};
use molly::{AWS_IOT_MQTT_ALPN, HELLO_WORLD_TOPIC, KEEP_ALIVE_INTERVAL};
use rumqttc::{AsyncClient, MqttOptions, QoS, TlsConfiguration, Transport};
use std::fs;
use std::{error::Error, time::Duration};


const CLIENT_ID: &str = "molly";

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

// fn main() -> Result<()> {
#[tokio::main(flavor = "current_thread")]

async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    molly::Configs::init();
    // print!("{:?}", molly::Configs::global());

    let name = CLIENT_ID;
    let endpoint = molly::Configs::global().iot_ats.to_string();
    // let ca = molly::Configs::global().ca.to_string();
    // let cert = molly::Configs::global().cert.to_string();
    // let key = molly::Configs::global().key.to_string();

    info!("{name} is connecting to {endpoint}");
    let mut mqttoptions = MqttOptions::new(name, endpoint, 8883);
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(KEEP_ALIVE_INTERVAL));

    let ca = include_bytes!("../certs/AmazonRootCA1.pem");
    let client_cert = include_bytes!("../certs/certificate.pem.crt");
    let client_key = include_bytes!("../certs/private.pem.key");

    let ca = ca.to_vec();

    let transport = Transport::Tls(TlsConfiguration::Simple {
        ca,
        alpn: None,
        client_auth: Some((client_cert.to_vec(), client_key.to_vec())),
    });

    mqttoptions.set_transport(transport);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client
    .subscribe(HELLO_WORLD_TOPIC, QoS::AtMostOnce)
    .await
    .unwrap();

    loop {
        let notification = match eventloop.poll().await {
            Ok(v) => {
                println!("Event = {v:?}");
            }
            Err(e) => {
                println!("Error = {e:?}");
                break;
            }
        };
        println!("Received = {:?}", notification);
    }

    Ok(())

    // let args = Cli::parse();
    // warn!("oops, nothing implemented!");


    // let content = std::fs::read_to_string(&args.path)
    //     .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    // molly::find_matches(&content, &args.pattern, &mut std::io::stdout());

    // Ok(())
}


#[test]
fn find_a_match() {
    let mut result = Vec::new();
    molly::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

