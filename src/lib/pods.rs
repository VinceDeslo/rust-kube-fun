use axum::response::{IntoResponse, Json};
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams},
    Client,
};
use tracing::info;

pub async fn list_pods() -> impl IntoResponse {
    Json("Pod Resource List")
}

pub async fn get_pod_resource() -> impl IntoResponse {
    Json("Pod Resource")
}

pub async fn print_pods(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let pods: Api<Pod> = Api::default_namespaced(client.clone());

    let list_params = ListParams::default();

    let pods_list = pods.list(&list_params).await?;

    let pod_count = pods_list.items.len();

    info!("Pod count: {}", pod_count);
    if pod_count <= 0 {
        return Ok(());
    }
    pods_list.into_iter().for_each(|pod| {
        match pod.metadata.name {
            Some(name) => info!("Found pod: {}", name),
            None => (),
        }
        match pod.metadata.uid {
            Some(uid) => info!("Pod UID: {}", uid),
            None => (),
        }
    });

    Ok(())
}
