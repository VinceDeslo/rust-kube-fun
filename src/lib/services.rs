use axum::response::{IntoResponse, Json};
use k8s_openapi::api::core::v1::Service;
use kube::{
    api::{Api, ListParams},
    Client,
};
use tracing::info;

pub async fn list_services() -> impl IntoResponse {
    Json("Service Resource List")
}

pub async fn get_service_resource() -> impl IntoResponse {
    Json("Service Resource")
}

pub async fn print_services(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let services: Api<Service> = Api::default_namespaced(client.clone());

    let list_params = ListParams::default();

    let services_list = services.list(&list_params).await?;

    let service_count = services_list.items.len();
    info!("Service count: {}", service_count);
    if service_count <= 0 {
        return Ok(());
    }

    services_list.into_iter().for_each(|service| {
        match service.metadata.name {
            Some(name) => info!("Found service: {}", name),
            None => (),
        }
        match service.metadata.uid {
            Some(uid) => info!("Service UID: {}", uid),
            None => (),
        }
    });

    Ok(())
}
