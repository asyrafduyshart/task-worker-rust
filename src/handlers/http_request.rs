use serde::Deserialize;

#[derive(Deserialize)]
pub struct HttpPostPayload {
    pub url: String,
    pub method: String,
    pub payload: String,
}
