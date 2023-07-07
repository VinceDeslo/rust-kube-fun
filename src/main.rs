use kube::Client;
use tracing::info;
use tracing_subscriber;

mod lib;
use lib::list::list_pods;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup tracing
    tracing_subscriber::fmt::init();

    // Create the k8s client
    info!("Creating k8s client");
    let client = Client::try_default().await?;

    // Read pods
    let _ = list_pods(client).await?;

    Ok(())
}

// async fn list_pods(client: Client) -> Result<(), Box<dyn std::error::Error>> {
//     let pods: Api<Pod> = Api::default_namespaced(client);
//
//     let list_params = ListParams::default();
//
//     let pods_list = pods.list(&list_params).await?;
//
//     pods_list.into_iter().for_each(|pod| {
//         match pod.metadata.name {
//             Some(name) => {
//                 info!("Found pod: {}", name);
//             }
//             None => info!("Failed to find pod name."),
//         }
//         match pod.metadata.uid {
//             Some(uid) => {
//                 info!("Pod UID: {:#?}", uid);
//             }
//             None => info!("Failed to find pod uid."),
//         }
//     });
//
//     Ok(())
// }
