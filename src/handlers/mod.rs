mod http_request;
mod kafka_request;
mod response;

use self::response::ApiResponse;
use hyper::{Body, Request, Response, StatusCode};
use kafka_request::KafkaPostPayload;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::to_string;

pub async fn handle(
    req: Request<Body>,
    producer: FutureProducer,
) -> Result<Response<Body>, hyper::Error> {
    let whole_body = hyper::body::to_bytes(req.into_body()).await?;
    let post_payload: KafkaPostPayload = match serde_json::from_slice(&whole_body) {
        Ok(payload) => payload,
        Err(e) => {
            let response = ApiResponse::new("failure", &format!("Failed to parse JSON: {}", e));
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(to_string(&response).unwrap()))
                .unwrap());
        }
    };

    let result = producer
        .send(
            FutureRecord::to(post_payload.topic.as_str())
                .payload(&post_payload.payload)
                .key(&post_payload.key),
            std::time::Duration::from_secs(5),
        )
        .await;

    match result {
        Ok(_) => {
            let response = ApiResponse::new("success", "Message sent successfully");
            Ok(Response::new(Body::from(to_string(&response).unwrap())))
        }
        Err(err) => {
            let (kr, _own) = err;
            let response: ApiResponse =
                ApiResponse::new("failure", &format!("{}", kr.to_string().as_str()));
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(to_string(&response).unwrap()))
                .unwrap())
        }
    }
}
