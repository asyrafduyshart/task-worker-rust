use std::time::Duration;

use hyper::{client::HttpConnector, Body, Client, Request, Response, StatusCode, Uri};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use serde_json::to_string;
use tokio::time::timeout;

use super::response::ApiResponse;

#[derive(Deserialize)]
pub struct HttpPostPayload {
    pub url: String,
    pub method: String,
    pub payload: String,
    pub timeout: u64,
}

pub async fn handle(
    req: Request<Body>,
    client: Client<HttpsConnector<HttpConnector>>,
) -> Result<Response<Body>, hyper::Error> {
    let whole_body = hyper::body::to_bytes(req.into_body()).await?;
    let post_payload: HttpPostPayload = match serde_json::from_slice(&whole_body) {
        Ok(payload) => payload,
        Err(e) => {
            let response = ApiResponse::new("failure", &format!("Failed to parse JSON: {}", e));
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(to_string(&response).unwrap()))
                .unwrap());
        }
    };

    let uri = post_payload.url.parse::<Uri>().unwrap();

    println!("uri: {:?}", uri);

    let request = Request::builder()
        .method(post_payload.method.as_str())
        .header("Content-Type", "application/json")
        .uri(uri)
        .body(Body::from(post_payload.payload))
        .unwrap();

    let response = client.request(request);
    let duration = Duration::from_secs(post_payload.timeout);
    let response = timeout(duration, response).await;

    match response {
        Ok(resp) => match resp {
            Ok(_) => {
                let response = ApiResponse::new("success", "Message sent successfully");
                return Ok(Response::new(Body::from(to_string(&response).unwrap())));
            }
            Err(err) => {
                let response: ApiResponse = ApiResponse::new(
                    "failure",
                    &format!("Failed to send request: {}", err.to_string().as_str()),
                );
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(to_string(&response).unwrap()))
                    .unwrap());
            }
        },
        Err(err) => {
            let response: ApiResponse =
                ApiResponse::new("failure", &format!("{}", err.to_string().as_str()));
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(to_string(&response).unwrap()))
                .unwrap());
        }
    }
}
