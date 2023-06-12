use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    status: String,
    message: String,
}

impl ApiResponse {
    pub fn new(status: &str, message: &str) -> Self {
        ApiResponse {
            status: status.to_string(),
            message: message.to_string(),
        }
    }
}
