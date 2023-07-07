use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams, ObjectMeta},
    Client,
};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup tracing
    tracing_subscriber::fmt::init();

    // Create the k8s client
    info!("Creating k8s client");
    let client = Client::try_default().await?;

    // Read pods
    let pods: Api<Pod> = Api::default_namespaced(client);

    let list_params = ListParams::default();

    for pod in pods.list(&list_params).await? {
        let meta: ObjectMeta = pod.metadata;

        match meta.name {
            Some(name) => {
                info!("Found pod: {}", name);
            }
            None => info!("Didn't find pod name."),
        }

        match meta.uid {
            Some(uid) => {
                info!("Pod UID: {:#?}", uid);
            }
            None => info!("Didn't find pod uid."),
        }
    }

    Ok(())
}
