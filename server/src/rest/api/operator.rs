//! Implementation of operator-related REST APIs.

use axum::{extract::Path, Extension, Json};
use hyper::StatusCode;
// use lib_common::grpc::ClientConnect;
// use svc_storage_client_grpc::prelude::*;

use super::is_uuid;
use super::structs::Operator;
use crate::grpc::client::GrpcClients;
use uuid::Uuid;

/// Get info about an operator by id.
#[utoipa::path(
    get,
    path = "/assets/operators/{id}",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Operator found in database", body = Operator),
        (status = 404, description = "Operator not found in database"),
        (status = 400, description = "Invalid operator id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Operator id"),
    )
)]
pub async fn get_operator(
    Extension(mut _grpc_clients): Extension<GrpcClients>,
    Path(operator_id): Path<String>,
) -> Result<Json<Operator>, (StatusCode, String)> {
    rest_info!("(get_operator) {}", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    // TODO(R4): let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("(get_operator) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    Ok(Json(Operator::random()))
}

#[utoipa::path(
    get,
    path = "/assets/operators/{id}/assets",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Assets found from database for operator {id}", body = [String]),
        (status = 404, description = "Operator not found in database"),
        (status = 400, description = "Invalid operator id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Operator id"),
    )
)]
/// Get all assets belonging to an operator.
pub async fn get_all_assets_by_operator(
    Extension(mut _grpc_clients): Extension<GrpcClients>,
    Path(operator_id): Path<String>,
) -> Result<Json<Vec<Uuid>>, (StatusCode, String)> {
    rest_info!("(get_all_assets_by_operator) {}", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    // let vertiport_client_option = grpc_clients.storage_vertiport.get_client().await;
    // let vertipad_client_option = grpc_clients.storage_vertipad.get_client().await;
    // if vertiport_client_option.is_none() || vertipad_client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("(get_all_assets) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }

    // let request = SearchFilter {
    //     search_field: "".to_string(),
    //     search_value: "".to_string(),
    //     page_number: 1,
    //     results_per_page: 50,
    // });

    // let mut vertiport_client = vertiport_client_option.unwrap();
    // let mut vertipad_client = vertipad_client_option.unwrap();
    // let mut result = Vec::new();
    // // Get Vertiports
    // let vertiports = vertiport_client
    //     .get_all_with_filter(request)
    //     .await
    //     .map_err(|e| {
    //         rest_error!("(get_all_assets) Error getting vertiports: {}", e);
    //         (
    //             StatusCode::SERVICE_UNAVAILABLE,
    //             "Error getting vertiports".to_string(),
    //         )
    //     })?
    //     .into_inner()
    //     .vertiports;
    // TODO(R4)
    Ok(Json(vec![]))
}
