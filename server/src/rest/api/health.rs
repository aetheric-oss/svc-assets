//! Health check REST API implementation.

use crate::grpc::client::GrpcClients;
use axum::Extension;
use hyper::StatusCode;
use lib_common::grpc::ClientConnect;

//===========================================================
// REST API Implementations
//===========================================================
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
    rest_debug!("(health_check) entry.");

    let mut ok = true;

    if grpc_clients.storage.vertiport.get_client().await.is_err() {
        let error_msg = "svc-storage vertiport unavailable.".to_string();
        rest_error!("(health_check) {}.", &error_msg);
        ok = false;
    }
    if grpc_clients.storage.vertipad.get_client().await.is_err() {
        let error_msg = "svc-storage vertipad unavailable.".to_string();
        rest_error!("(health_check) {}.", &error_msg);
        ok = false;
    }
    if grpc_clients.storage.vehicle.get_client().await.is_err() {
        let error_msg = "svc-storage vehicle unavailable.".to_string();
        rest_error!("(health_check) {}.", &error_msg);
        ok = false;
    }

    match ok {
        true => {
            rest_debug!("(health_check) healthy, all dependencies running.");
            Ok(())
        }
        false => {
            rest_error!("(health_check) unhealthy, 1+ dependencies down.");
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}
