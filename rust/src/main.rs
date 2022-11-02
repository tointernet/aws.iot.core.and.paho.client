use futures_util::StreamExt;
use mqtt::{AsyncClient, ConnectOptions, MessageBuilder, SslOptionsBuilder, SslVersion};
use paho_mqtt as mqtt;
use std::{env, time::Duration};
use tracing::{debug, error, info};
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

#[tokio::main]
async fn main() -> Result<(), ()> {
    logger()?;

    let (mut mqtt_client, conn_opts) = mqtt_client()?;

    debug!("connection to mqtt...");

    let mut stream = mqtt_client.get_stream(2048);

    mqtt_client.connect(conn_opts.clone()).await.map_err(|e| {
        error!(error = e.to_string(), "error to connect");
        ()
    })?;

    debug!("mqtt was connected");

    publisher(&mqtt_client);

    let topic = env::var("AWS_IOT_TOPIC_TO_SUBSCRIBE").unwrap();
    mqtt_client.subscribe(topic, 0).await.map_err(|e| {
        error!(error = e.to_string(), "error to subscribe");
        ()
    })?;

    while let Some(delivery) = stream.next().await {
        match delivery {
            Some(_msg) => info!("received mqtt msg"),
            _ => {}
        }
    }

    Ok(())
}

fn logger() -> Result<(), ()> {
    tracing::subscriber::set_global_default(tracing_subscriber::registry().with(
        BunyanFormattingLayer::new("aws-broker".to_owned(), std::io::stdout),
    ))
    .map_err(|_| ())?;

    Ok(())
}

fn mqtt_client() -> Result<(AsyncClient, ConnectOptions), ()> {
    debug!("creating to mqtt client...");

    let server_uri = env::var("AWS_IOT_DEVICE_DATA_ENDPOINT").unwrap();
    let client_id = env::var("AWS_IOT_DEVICE_NAME").unwrap();
    let trust_store = env::var("AWS_ROOT_CA_PATH").unwrap();
    let key_store = env::var("AWS_THING_CERT_PATH").unwrap();
    let private_key = env::var("AWS_THING_PRIVATE_KEY_PATH").unwrap();

    let opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(server_uri)
        .client_id(client_id)
        .finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(10))
        .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
        .clean_session(true)
        .ssl_options(
            SslOptionsBuilder::new()
                .alpn_protos(&["x-amzn-mqtt-ca"])
                .trust_store(trust_store)
                .unwrap()
                .key_store(key_store)
                .unwrap()
                .private_key(private_key)
                .unwrap()
                .ssl_version(SslVersion::Tls_1_2)
                .verify(true)
                .finalize(),
        )
        .finalize();

    let client = mqtt::AsyncClient::new(opts).map_err(|e| {
        error!(error = e.to_string(), "error to create mqtt client");
        ()
    })?;

    debug!("mqtt client was created");

    Ok((client, conn_opts))
}

fn publisher(client: &AsyncClient) {
    let topic = env::var("AWS_IOT_TOPIC_TO_PUBLISH").unwrap();

    tokio::spawn({
        let clone = client.clone();

        async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;

                debug!("publishing to mqtt...");

                match clone
                    .clone()
                    .publish(
                        MessageBuilder::new()
                            .topic(&topic)
                            .payload(vec![])
                            .qos(1)
                            .finalize(),
                    )
                    .await
                {
                    Ok(_) => {
                        debug!("published to mqtt");
                    }
                    Err(e) => {
                        error!(error = e.to_string(), "error to publish");
                    }
                }
            }
        }
    });
}
