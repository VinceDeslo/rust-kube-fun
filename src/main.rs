use kube::Client;
use tracing::info;
use tracing_subscriber;

mod lib;
use lib::pods::print_pods;
use lib::services::print_services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    info!("Creating k8s client");
    let client = Client::try_default().await?;

    print_pods(&client).await?;
    print_services(&client).await?;

    Ok(())
}
