use kube::Client;
use tracing::info;
use tracing_subscriber;

mod lib;
use lib::pods::print_pods;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup tracing
    tracing_subscriber::fmt::init();

    // Create the k8s client
    info!("Creating k8s client");
    let client = Client::try_default().await?;

    // Print pods
    print_pods(client).await?;

    Ok(())
}
