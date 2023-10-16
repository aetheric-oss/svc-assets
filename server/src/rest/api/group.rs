//! Implementation of asset group related REST APIs.

use axum::{extract::Path, Extension, Json};
use hyper::StatusCode;

use super::is_uuid;
use super::rest_types::RegisterAssetGroupPayload;
use super::structs::AssetGroup;
use crate::grpc::client::GrpcClients;
use uuid::Uuid;

//-----------------------------------------------------------
// Group management
//-----------------------------------------------------------

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

    let _asset_group = AssetGroup {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        owner: payload.owner,
        created_at: None,
        updated_at: None,
        delegatee: None,
        assets: payload.assets,
    };

    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("(register_asset_group) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    // TODO(R4)
    Ok(_asset_group.id)
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

    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("(remove_asset_group) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    // TODO(R4)
    Ok(_id)
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

    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("(update_asset_group) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    // TODO(R4)
    Ok(payload.id)
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
) -> Result<Json<Vec<Uuid>>, (StatusCode, String)> {
    rest_info!("(get_all_grouped_assets) {}", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("(get_all_grouped_assets) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    // TODO(R4)
    Ok(Json(vec![]))
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
) -> Result<Json<Vec<Uuid>>, (StatusCode, String)> {
    rest_info!("(get_all_grouped_assets_delegated_to) {}", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("(get_all_grouped_assets_delegated_to) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    // TODO(R4)
    Ok(Json(vec![]))
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
) -> Result<Json<Vec<Uuid>>, (StatusCode, String)> {
    rest_info!("(get_all_grouped_assets_delegated_from) {}", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("(get_all_grouped_assets_delegated_from) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    // TODO(R4)
    Ok(Json(vec![]))
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
    if !is_uuid(&asset_group_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid asset group id".to_string(),
        ));
    }
    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("(get_asset_group_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    // TODO(R4)
    Ok(Json(AssetGroup::random()))
}
