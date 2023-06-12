use std::env;

use dotenv::dotenv;
use rdkafka::{producer::FutureProducer, ClientConfig};

pub fn create_producer() -> FutureProducer {
    dotenv().ok();

    let kafka_broker = env::var("KAFKA_BROKER").expect("KAFKA_BROKER must be set");
    let sasl_username = env::var("KAFKA_KEY").expect("KAFKA_KEY must be set");
    let sasl_password = env::var("KAFKA_SECRET").expect("KAFKA_SECRET must be set");

    let producer = ClientConfig::new()
        .set("bootstrap.servers", &kafka_broker)
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "PLAIN")
        .set("sasl.username", &sasl_username)
        .set("sasl.password", &sasl_password)
        .create()
        .expect("Producer creation error");

    return producer;
}
