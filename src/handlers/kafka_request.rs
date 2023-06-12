use serde::Deserialize;

#[derive(Deserialize)]
pub struct KafkaPostPayload {
    pub topic: String,
    pub key: String,
    pub payload: String,
}
