use futures_util::StreamExt;
use mqtt::{AsyncClient, ConnectOptions, MessageBuilder, SslOptionsBuilder};
use paho_mqtt as mqtt;
use std::time::Duration;
use tracing::{error, info};
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

#[tokio::main]
async fn main() -> Result<(), ()> {
    logger()?;

    let (mut mqtt_client, conn_opts) = mqtt_client()?;

    let mut stream = mqtt_client.get_stream(2048);

    mqtt_client.connect(conn_opts.clone()).await.map_err(|e| {
        error!(error = e.to_string(), "error to create mqtt client");
        ()
    })?;

    publisher(&mqtt_client);

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
    let opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("")
        .client_id("")
        .finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(60))
        .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
        .clean_session(false)
        .user_name("")
        .password("")
        .ssl_options(
            SslOptionsBuilder::new()
                .ca_path("./aws-root-ca.crt")
                .unwrap()
                .key_store("./aws-thing-cert.pem")
                .unwrap()
                .private_key("./aws-thing-private.key")
                .unwrap()
                .private_key_password("")
                .enable_server_cert_auth(false)
                .verify(false)
                .finalize(),
        )
        .finalize();

    let client = mqtt::AsyncClient::new(opts).map_err(|e| {
        error!(error = e.to_string(), "error to create mqtt client");
        ()
    })?;

    Ok((client, conn_opts))
}

fn publisher(client: &AsyncClient) {
    tokio::spawn({
        let clone = client.clone();

        async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;

                match clone
                    .clone()
                    .publish(
                        MessageBuilder::new()
                            .topic("/test")
                            .payload(vec![])
                            .qos(1)
                            .finalize(),
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        error!(error = e.to_string(), "error to publish");
                    }
                }
            }
        }
    });
}
