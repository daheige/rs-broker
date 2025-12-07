use dotenv::dotenv;
use env_logger;
use log::{error, info, warn};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::Headers;
use rdkafka::{ClientConfig, Message};
use std::env;

// 命令终端运行方式：RUST_LOG=debug cargo run --bin consumer
#[tokio::main]
async fn main() {
    // 初始化logger配置
    // 日志level 优先级  error > warn > info > debug > trace
    // 设置日志级别环境变量，这里注释掉了，启动的时可手动指定RUST_LOG=debug
    // unsafe {
    //     env::set_var("RUST_LOG", "debug");
    // }
    env_logger::init();

    dotenv().ok().expect(".env file not found");
    let brokers = env::var("KAFKA_BROKERS").expect("KAFKA_BROKERS not set");

    println!("consumer message begin...");
    // 消费消息
    let topic = "my-topic";
    let group = "group-1";
    consume_and_print(&brokers, group, &[topic], None).await;
}

async fn consume_and_print(
    brokers: &str,
    group_id: &str,
    topics: &[&str],
    assignor: Option<&String>,
) {
    let mut config = ClientConfig::new();

    config
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("statistics.interval.ms", "30000")
        // 从分区的最小偏移量（最早消息）开始消费
        // .set("auto.offset.reset", "earliest")
        // 从分区的最大偏移量（最新消息）开始消费
        .set("auto.offset.reset", "largest")
        .set_log_level(RDKafkaLogLevel::Debug);

    if let Some(assignor) = assignor {
        config
            .set("group.remote.assignor", assignor)
            .set("group.protocol", "consumer")
            .remove("session.timeout.ms");
    }

    let consumer: StreamConsumer = config.create().unwrap();

    consumer
        .subscribe(topics)
        .expect("Can't subscribe to specified topics");

    loop {
        match consumer.recv().await {
            Err(e) => error!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                println!(
                    "key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                    m.key(),
                    payload,
                    m.topic(),
                    m.partition(),
                    m.offset(),
                    m.timestamp()
                );

                println!("receive msg: {}", payload);
                if let Some(headers) = m.headers() {
                    for header in headers.iter() {
                        info!("  Header {:#?}: {:?}", header.key, header.value);
                    }
                }

                // 提交消息ack
                consumer
                    .commit_message(&m, CommitMode::Async)
                    .expect("failed to commit ack");
            }
        };
    }
}
