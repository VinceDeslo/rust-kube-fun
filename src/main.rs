use axum::{routing::get, Router};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use kube::Client;
use tracing::info;
use tracing_subscriber;

mod lib;
// use lib::pods::print_pods;
use lib::pods::{get_pod_resource, list_pods};
// use lib::services::print_services;
use lib::services::{get_service_resource, list_services};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ipv4 = Ipv4Addr::new(127, 0, 0, 1);
    let ip = IpAddr::V4(ipv4);
    let addr = SocketAddr::new(ip, 3000);

    let app = Router::new()
        .route("/pods", get(list_pods))
        .route("/pod", get(get_pod_resource))
        .route("/services", get(list_services))
        .route("/service", get(get_service_resource));

    tracing_subscriber::fmt::init();

    info!("Creating k8s client");
    let _client = Client::try_default().await?;

    info!("Listening on address: {:?}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    // print_pods(&client).await?;
    // print_services(&client).await?;

    Ok(())
}
