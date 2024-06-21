//! Handlers for actions on operator records

/// Types Used in REST Messages
pub use super::rest_types::*;

use axum::{extract::Path, Extension, Json};
use hyper::StatusCode;

use crate::grpc::client::GrpcClients;
use lib_common::time::{DateTime, Utc};
use lib_common::uuid::{to_uuid, Uuid};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// A struct representing the operator.
///
/// An operator is a legal entity that owns assets and is a customer of
/// Arrow Cargo. The operator supplies the assets to the network,
/// expects to receive and operate cargo shipments, and is expected to
/// derive revenue from the operation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Operator {
    /// UUID of the operator.
    pub id: String,

    /// The name of the operator.
    pub name: String,

    /// The country where the operator is located.
    pub country: String,

    /// The city where the operator is located.
    pub city: String,

    /// The address of the operator.
    pub address: String,

    /// The postal code of the operator.
    pub postal_code: String,

    /// The email of the operator.
    pub email: String,

    /// The phone number of the operator.
    pub phone: String,

    /// The website of the operator.
    pub website: String,

    /// The description of the operator.
    pub description: String,

    /// The logo of the operator.
    pub logo: String,

    /// The created_at timestamp.
    pub created_at: DateTime<Utc>,

    /// The updated_at timestamp.
    pub updated_at: DateTime<Utc>,
}

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
) -> Result<Json<Operator>, StatusCode> {
    rest_info!("{}", operator_id);
    let _id = to_uuid(&operator_id).ok_or_else(|| {
        rest_error!("Invalid operator id.");
        StatusCode::BAD_REQUEST
    })?;

    Err(StatusCode::NOT_IMPLEMENTED)
}

//-----------------------------------------------------------
// Get assets by operator
//-----------------------------------------------------------
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
) -> Result<Json<Vec<Uuid>>, StatusCode> {
    rest_info!("{}", operator_id);
    let _id = to_uuid(&operator_id).ok_or_else(|| {
        rest_error!("Invalid operator id.");
        StatusCode::BAD_REQUEST
    })?;

    Err(StatusCode::NOT_IMPLEMENTED)

    // Get Client
    // let vertiport_client_option = grpc_clients.storage_vertiport.get_client().await;
    // let vertipad_client_option = grpc_clients.storage_vertipad.get_client().await;
    // if vertiport_client_option.is_none() || vertipad_client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("{}", &error_msg);
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
    //         rest_error!("Error getting vertiports: {}", e);
    //         (
    //             StatusCode::SERVICE_UNAVAILABLE,
    //             "Error getting vertiports".to_string(),
    //         )
    //     })?
    //     .into_inner()
    //     .vertiports;
    // TODO(R5)
    // Ok(Json(vec![]))
}

/// Get all grouped assets belonging to an operator.
///
/// These are the assets NOT being delegated to or from this operator.
///
/// Returns a list of grouped asset ids.
#[utoipa::path(
    get,
    path = "/assets/operators/{id}/grouped",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Grouped assets found from database for operator {id}", body = [String]),
        (status = 404, description = "Operator not found in database"),
        (status = 400, description = "Invalid operator id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Operator id"),
    )
)]
pub async fn get_all_grouped_assets(
    Extension(mut _grpc_clients): Extension<GrpcClients>,
    Path(operator_id): Path<String>,
) -> Result<Json<Vec<Uuid>>, StatusCode> {
    rest_info!("{}", operator_id);

    let _id = to_uuid(&operator_id).ok_or_else(|| {
        rest_error!("Invalid operator id.");
        StatusCode::BAD_REQUEST
    })?;

    Err(StatusCode::NOT_IMPLEMENTED)

    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("{}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    // TODO(R5)
    // Ok(Json(vec![]))
}

/// Get all grouped assets delegated to an operator.
#[utoipa::path(
    get,
    path = "/assets/operators/{id}/grouped/delegated-to",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Grouped assets delegated to operator {id} found from database", body = [String]),
        (status = 404, description = "Operator not found in database"),
        (status = 400, description = "Invalid operator id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Operator id"),
    )
)]
pub async fn get_all_grouped_assets_delegated_to(
    Extension(mut _grpc_clients): Extension<GrpcClients>,
    Path(operator_id): Path<String>,
) -> Result<Json<Vec<Uuid>>, StatusCode> {
    rest_info!("{}", operator_id);
    let _id = to_uuid(&operator_id).ok_or_else(|| {
        rest_error!("Invalid operator id.");
        StatusCode::BAD_REQUEST
    })?;

    Err(StatusCode::NOT_IMPLEMENTED)

    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("{}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    // TODO(R5)
    // Ok(Json(vec![]))
}

/// Get all grouped assets delegated from an operator.
#[utoipa::path(
    get,
    path = "/assets/operators/{id}/grouped/delegated-from",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Grouped assets delegated from operator {id} found from database", body = [String]),
        (status = 404, description = "Operator not found in database"),
        (status = 400, description = "Invalid operator id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Operator id"),
    )
)]
pub async fn get_all_grouped_assets_delegated_from(
    Extension(mut _grpc_clients): Extension<GrpcClients>,
    Path(operator_id): Path<String>,
) -> Result<Json<Vec<Uuid>>, StatusCode> {
    rest_info!("{}", operator_id);

    let _id = to_uuid(&operator_id).ok_or_else(|| {
        rest_error!("Invalid operator id.");
        StatusCode::BAD_REQUEST
    })?;

    Err(StatusCode::NOT_IMPLEMENTED)

    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("{}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    // TODO(R5)
    // Ok(Json(vec![]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib_common::uuid::Uuid;

    #[tokio::test]
    async fn test_get_operator() {
        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        // invalid id
        let id = "invalid".to_string();
        let result = get_operator(Extension(grpc_clients.clone()), Path(id))
            .await
            .unwrap_err();
        assert_eq!(result, StatusCode::BAD_REQUEST);

        // valid id
        let id = Uuid::new_v4().to_string();
        let result = get_operator(Extension(grpc_clients.clone()), Path(id))
            .await
            .unwrap_err();
        assert_eq!(result, StatusCode::NOT_IMPLEMENTED);
    }

    #[tokio::test]
    async fn test_get_all_assets_by_operator() {
        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        // invalid id
        let id = "invalid".to_string();
        let result = get_all_assets_by_operator(Extension(grpc_clients.clone()), Path(id))
            .await
            .unwrap_err();
        assert_eq!(result, StatusCode::BAD_REQUEST);

        // valid id
        let id = Uuid::new_v4().to_string();
        let result = get_all_assets_by_operator(Extension(grpc_clients.clone()), Path(id))
            .await
            .unwrap_err();
        assert_eq!(result, StatusCode::NOT_IMPLEMENTED);
    }

    #[tokio::test]
    async fn test_get_all_grouped_assets() {
        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        // invalid id
        let id = "invalid".to_string();
        let result = get_all_grouped_assets(Extension(grpc_clients.clone()), Path(id))
            .await
            .unwrap_err();
        assert_eq!(result, StatusCode::BAD_REQUEST);

        // valid id
        let id = Uuid::new_v4().to_string();
        let result = get_all_grouped_assets(Extension(grpc_clients.clone()), Path(id))
            .await
            .unwrap_err();
        assert_eq!(result, StatusCode::NOT_IMPLEMENTED);
    }

    #[tokio::test]
    async fn test_get_all_grouped_assets_delegated_to() {
        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        // invalid id
        let id = "invalid".to_string();
        let result = get_all_grouped_assets_delegated_to(Extension(grpc_clients.clone()), Path(id))
            .await
            .unwrap_err();
        assert_eq!(result, StatusCode::BAD_REQUEST);

        // valid id
        let id = Uuid::new_v4().to_string();
        let result = get_all_grouped_assets_delegated_to(Extension(grpc_clients.clone()), Path(id))
            .await
            .unwrap_err();
        assert_eq!(result, StatusCode::NOT_IMPLEMENTED);
    }

    #[tokio::test]
    async fn test_get_all_grouped_assets_delegated_from() {
        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        // invalid id
        let id = "invalid".to_string();
        let result =
            get_all_grouped_assets_delegated_from(Extension(grpc_clients.clone()), Path(id))
                .await
                .unwrap_err();
        assert_eq!(result, StatusCode::BAD_REQUEST);

        // valid id
        let id = Uuid::new_v4().to_string();
        let result =
            get_all_grouped_assets_delegated_from(Extension(grpc_clients.clone()), Path(id))
                .await
                .unwrap_err();
        assert_eq!(result, StatusCode::NOT_IMPLEMENTED);
    }
}
