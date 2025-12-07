use rust_broker::Broker;
use rust_broker::{KafkaConfig, new_broker};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok().expect(".env file not found");
    let brokers = env::var("KAFKA_BROKERS").expect("KAFKA_BROKERS not set");

    // kafka config
    let kafka_config = KafkaConfig::builder(&brokers)
        .with_security_protocol("PLAINTEXT")
        .build();

    // create kafka broker
    let broker = Arc::new(new_broker(kafka_config).expect("Could not create Kafka broker"));
    let broker_clone = broker.clone();
    let topic = "my-topic";
    let handler = tokio::spawn(async move {
        let delivery_status = broker_clone.publish(topic, "hello,world".as_bytes()).await;
        if let Err(err) = delivery_status {
            println!("Error sending message: {:?}", err);
            return;
        }

        println!("delivery status: {:?}", delivery_status);
    });
    handler.await.unwrap();

    // shutdown broker
    let _ = broker.shutdown().await;
}
