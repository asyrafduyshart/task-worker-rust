use crate::config::kafka::create_producer;
use crate::handlers::handle;
use hyper::{Body, Method, Request, Response, StatusCode};

pub async fn route(req: Request<Body>) -> Result<hyper::Response<Body>, hyper::Error> {
    let producer = create_producer();

    match (req.method(), req.uri().path()) {
        (&Method::POST, "/task/kafka") => handle(req, producer).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap()),
    }
}
