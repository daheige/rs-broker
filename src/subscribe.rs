use dotenv::dotenv;
use rs_broker::Broker;
use rs_broker::{KafkaConfig, new_broker};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok().expect(".env file not found");
    let brokers = env::var("KAFKA_BROKERS").expect("KAFKA_BROKERS not set");

    // kafka config
    let kafka_config = KafkaConfig::builder(&brokers)
        .with_security_protocol("PLAINTEXT")
        .build();

    // create kafka broker
    let broker = new_broker(kafka_config).expect("Could not create Kafka broker");
    let topic = "my-topic";
    let group = "group-1";
    println!("consume message begin...");
    broker
        .subscribe(topic, group, |payload| {
            let message = String::from_utf8_lossy(&payload).to_string();
            async move {
                println!("handler msg");
                let _ = handler(&message).await;
                // return ok
                Ok(())
            }
        })
        .await
        .expect("Could not subscribe to topic");
}

async fn handler(msg: &str) -> Result<(), String> {
    println!("Got msg len: {}", msg.len());
    println!("consumer msg: {}", msg);
    Ok(())
}
