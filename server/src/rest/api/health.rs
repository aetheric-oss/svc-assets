//! Health check REST endpoint

use crate::grpc::client::GrpcClients;
use axum::Extension;
use hyper::StatusCode;
use svc_storage_client_grpc::prelude::ReadyRequest;
use svc_storage_client_grpc::simple_service::Client;

/// Health check for load balancing
#[utoipa::path(
    get,
    path = "/health",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Service is healthy, all dependencies running."),
        (status = 503, description = "Service is unhealthy, one or more dependencies unavailable.")
    )
)]
pub async fn health_check(
    Extension(grpc_clients): Extension<GrpcClients>,
) -> Result<(), StatusCode> {
    rest_debug!("entry.");

    let mut ok = true;

    if grpc_clients
        .storage
        .vertiport
        .is_ready(ReadyRequest {})
        .await
        .is_err()
    {
        let error_msg = "svc-storage vertiport unavailable.".to_string();
        rest_error!("{}.", &error_msg);
        ok = false;
    }

    if grpc_clients
        .storage
        .vertipad
        .is_ready(ReadyRequest {})
        .await
        .is_err()
    {
        let error_msg = "svc-storage vertipad unavailable.".to_string();
        rest_error!("{}.", &error_msg);
        ok = false;
    }

    if grpc_clients
        .storage
        .vehicle
        .is_ready(ReadyRequest {})
        .await
        .is_err()
    {
        let error_msg = "svc-storage vehicle unavailable.".to_string();
        rest_error!("{}.", &error_msg);
        ok = false;
    }

    match ok {
        true => {
            rest_debug!("healthy, all dependencies running.");
            Ok(())
        }
        false => {
            rest_error!("unhealthy, 1+ dependencies down.");
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);
        health_check(Extension(grpc_clients)).await.unwrap();
    }
}
