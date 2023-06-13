use env_logger;
use log::{debug, info};
use std::env;

#[test]
fn test_log() {
    // Setting environment variables manually
    env::set_var("RUST_LOG", "logger=debug");
    env_logger::init();

    println!("Hello, broker!");
    info!("hello");
    debug!("broker");
    println!("{}", "kafka-broker");
}
