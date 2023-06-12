mod config;
mod handlers;
mod router;

use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use crate::router::route;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();
    // Retrieve the IP address from the environment variable "IP"
    let ip_str = env::var("LISTEN_IP").unwrap_or_else(|_| String::from("127.0.0.1"));
    let ip: Ipv4Addr = ip_str
        .parse()
        .expect("Listen IP must be in the format x.x.x.x");

    // Retrieve the port from the environment variable "PORT"
    let port_str = env::var("APP_PORT").unwrap_or_else(|_| String::from("8050"));
    let port: u16 = port_str.parse().expect("PORT must be a number");

    let addr = SocketAddr::new(IpAddr::V4(ip), port);

    let make_svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(route)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
