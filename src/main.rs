mod config;
mod handlers;
mod router;

use crate::router::route;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 8050).into();

    let make_svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(route)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
