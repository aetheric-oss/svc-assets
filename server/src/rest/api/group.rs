//! Handlers for the asset group API endpoints.

pub use super::rest_types::*;

use axum::{extract::Path, Extension, Json};
use hyper::StatusCode;

use crate::grpc::client::GrpcClients;
use crate::rest::structs::AssetGroup;

/// Register an [`AssetGroup`](crate::rest::structs::AssetGroup) in the database.
#[utoipa::path(
    post,
    path = "/assets/groups",
    tag = "svc-assets",
    request_body=RegisterAssetGroupPayload,
    responses(
        (status = 200, description = "AssetGroup registered in database; a UUID is returned", body = String),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn register_asset_group(
    Extension(mut _grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<RegisterAssetGroupPayload>,
) -> Result<String, (StatusCode, String)> {
    rest_info!("(register_asset_group) with payload: {:?}", &payload);
    Err((StatusCode::NOT_IMPLEMENTED, "Not implemented".to_string()))

    // let _asset_group = AssetGroup {
    //     id: Uuid::new_v4().to_string(),
    //     name: payload.name,
    //     owner: payload.owner,
    //     created_at: None,
    //     updated_at: None,
    //     delegatee: None,
    //     assets: payload.assets,
    // };

    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("(get_asset_group_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    // TODO(R4)
    // Ok(_asset_group.id)
}

/// Update/modify an [`AssetGroup`](crate::rest::structs::AssetGroup) in the database.
#[utoipa::path(
    put,
    path = "/assets/groups/{id}",
    tag = "svc-assets",
    request_body=AssetGroup,
    responses(
        (status = 200, description = "AssetGroup updated in database; a UUID is returned", body = String),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "AssetGroup id"),
    )
)]
pub async fn update_asset_group(
    Extension(mut _grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<AssetGroup>,
    Path(_id): Path<String>,
) -> Result<String, (StatusCode, String)> {
    rest_info!("(update_asset_group) with payload: {:?}", &payload);
    Err((StatusCode::NOT_IMPLEMENTED, "Not implemented".to_string()))

    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("(get_asset_group_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    // TODO(R4)
    // Ok(payload.id)
}

/// Remove an [`AssetGroup`](crate::rest::structs::AssetGroup) from the database.
#[utoipa::path(
    delete,
    path = "/assets/groups/{id}",
    tag = "svc-assets",
    responses(
        (status = 200, description = "AssetGroup removed from database; a UUID is returned", body = String),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "AssetGroup id"),
    )
)]
pub async fn remove_asset_group(
    Extension(_grpc_clients): Extension<GrpcClients>,
    Path(_id): Path<String>,
) -> Result<String, (StatusCode, String)> {
    rest_info!("(remove_asset_group) with id: {:?}", &_id);
    Err((StatusCode::NOT_IMPLEMENTED, "Not implemented".to_string()))

    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("(remove_asset_group) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    // TODO(R4)
    // Ok(_id)
}

/// Get an [`AssetGroup`](crate::rest::structs::AssetGroup) by its id.
#[utoipa::path(
    get,
    path = "/assets/groups/{id}",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Asset group {id} found from database", body = AssetGroup),
        (status = 404, description = "Asset group not found in database"),
        (status = 400, description = "Invalid asset group id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Asset group id"),
    )
)]
pub async fn get_asset_group_by_id(
    Extension(mut _grpc_clients): Extension<GrpcClients>,
    Path(asset_group_id): Path<String>,
) -> Result<Json<AssetGroup>, (StatusCode, String)> {
    rest_info!("(get_asset_group_by_id) {}", asset_group_id);

    // let _asset_group_id = to_uuid(&asset_group_id).ok_or_else(|| {
    //     rest_error!("(get_asset_group_by_id) Invalid asset group id.");
    //     (
    //         StatusCode::BAD_REQUEST,
    //         "Invalid asset group id".to_string(),
    //     )
    // })?;

    Err((StatusCode::NOT_IMPLEMENTED, "Not implemented".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib_common::logger::get_log_handle;
    use lib_common::uuid::Uuid;

    #[tokio::test]
    async fn test_register_asset_group() {
        get_log_handle().await;
        ut_info!("(test_register_asset_group) Start.");

        let payload = RegisterAssetGroupPayload {
            name: Some("Test Group".to_string()),
            owner: "Test Owner".to_string(),
            assets: vec![],
        };
        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);
        let result = register_asset_group(Extension(grpc_clients), Json(payload)).await;
        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
    }

    #[tokio::test]
    async fn test_update_asset_group() {
        get_log_handle().await;
        ut_info!("(test_update_asset_group) Start.");

        let payload = AssetGroup {
            id: Uuid::new_v4().to_string(),
            name: Some("Test Group".to_string()),
            owner: "Test Owner".to_string(),
            created_at: None,
            updated_at: None,
            delegatee: None,
            assets: vec![],
        };

        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);
        let result = update_asset_group(
            Extension(grpc_clients),
            Json(payload),
            Path(Uuid::new_v4().to_string()),
        )
        .await;
        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
    }

    #[tokio::test]
    async fn test_remove_asset_group() {
        get_log_handle().await;
        ut_info!("(test_remove_asset_group) Start.");

        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);
        let result =
            remove_asset_group(Extension(grpc_clients), Path(Uuid::new_v4().to_string())).await;
        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
    }

    #[tokio::test]
    async fn test_get_asset_group_by_id() {
        get_log_handle().await;
        ut_info!("(test_get_asset_group_by_id) Start.");

        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);
        let result =
            get_asset_group_by_id(Extension(grpc_clients), Path(Uuid::new_v4().to_string())).await;
        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
    }
}
