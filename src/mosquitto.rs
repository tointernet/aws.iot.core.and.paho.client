use futures_util::StreamExt;
use mqtt::{AsyncClient, ConnectOptions, MessageBuilder, SslOptionsBuilder, SslVersion};
use paho_mqtt as mqtt;
use std::time::Duration;
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
        error!(error = e.to_string(), "error to create mqtt client");
        ()
    })?;

    debug!("mqtt was connected");

    publisher(&mqtt_client);

    mqtt_client.subscribe("/topic/#", 1).await.map_err(|e| {
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
        BunyanFormattingLayer::new("broker".to_owned(), std::io::stdout),
    ))
    .map_err(|_| ())?;

    Ok(())
}

fn mqtt_client() -> Result<(AsyncClient, ConnectOptions), ()> {
    debug!("creating to mqtt client...");

    let opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("ssl://test.mosquitto.org:8884")
        .client_id("rust_client_id")
        .mqtt_version(4)
        .finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(10))
        .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
        .clean_session(false)
        .ssl_options(
            SslOptionsBuilder::new()
                .ca_path("/home/ralvescosta/Desktop/ToI/aws/mqtt-broker-test/mosquitto.org.pem")
                .unwrap()
                .trust_store(
                    "/home/ralvescosta/Desktop/ToI/aws/mqtt-broker-test/mosquitto.test.client.pem",
                )
                .unwrap()
                // .key_store(
                //     "/home/ralvescosta/Desktop/ToI/aws/mqtt-broker-test/mosquitto.test.client.pem",
                // )
                // .unwrap()
                .private_key(
                    "/home/ralvescosta/Desktop/ToI/aws/mqtt-broker-test/mosquitto.test.private.key",
                )
                .unwrap()
                .private_key_password("")
                .enable_server_cert_auth(false)
                .ssl_version(SslVersion::Tls_1_2)
                .verify(false)
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
                            .topic("/test/first")
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
