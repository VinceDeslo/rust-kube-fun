use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams},
    Client,
};
use tracing::info;

pub async fn list_pods(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    let pods: Api<Pod> = Api::default_namespaced(client);

    let list_params = ListParams::default();

    let pods_list = pods.list(&list_params).await?;

    pods_list.into_iter().for_each(|pod| {
        match pod.metadata.name {
            Some(name) => {
                info!("Found pod: {}", name);
            }
            None => info!("Failed to find pod name."),
        }
        match pod.metadata.uid {
            Some(uid) => {
                info!("Pod UID: {:#?}", uid);
            }
            None => info!("Failed to find pod uid."),
        }
    });

    Ok(())
}
