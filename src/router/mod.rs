use crate::config::kafka::create_producer;
use crate::handlers::http::handle as http_handle;
use crate::handlers::kafka::handle as kafka_handle;
use hyper::{Body, Client, Method, Request, Response, StatusCode};
use hyper_tls::HttpsConnector;

pub async fn route(req: Request<Body>) -> Result<hyper::Response<Body>, hyper::Error> {
    // create kafka broker producer
    let producer = create_producer();

    // create http client
    // Create a new HTTP client to send requests
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from("Rust Task Worker!"))),
        (&Method::POST, "/task/kafka") => kafka_handle(req, producer).await,
        (&Method::POST, "/task/http") => http_handle(req, client).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap()),
    }
}
