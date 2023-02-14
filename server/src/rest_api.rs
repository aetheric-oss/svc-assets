//! Implementation of REST APIs.

/// Types Used in REST Messages
pub mod rest_types {
    include!("../../openapi/types.rs");
}
use std::str::FromStr;

use prost_types::{FieldMask, Timestamp};
pub use rest_types::*;

use axum::{extract::Path, Extension, Json};
use chrono::Utc;
use hyper::StatusCode;
use svc_storage_client_grpc::{
    client::AdvancedSearchFilter,
    client::Id,
    vehicle::{Data, UpdateObject},
    vertipad::{Data as VertipadData, UpdateObject as VertipadUpdateObject},
    vertiport::{Data as VertiportData, UpdateObject as VertiportUpdateObject},
};
use uuid::Uuid;

use crate::{
    grpc_clients::GrpcClients, req_debug, req_error, structs::Aircraft, structs::AssetGroup,
    structs::Operator, structs::Vertipad, structs::Vertiport,
};

//===========================================================
// Helpers
//===========================================================

/// Check if a string is a valid UUID.
fn is_uuid(s: &str) -> bool {
    uuid::Uuid::try_parse(s).is_ok()
}

//===========================================================
// REST API Implementations
//===========================================================

/// Get info about an operator by id.
#[utoipa::path(
    get,
    path = "/assets/operators/{id}",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Operator found in database"),
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
    req_debug!("get_operator({})", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    // TODO let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_operator) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    //TODO
    Ok(Json(Operator::random()))
}

//-----------------------------------------------------------
// R2 DEMO
//-----------------------------------------------------------

#[utoipa::path(
    post,
    path = "/assets/demo/aircraft",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Assets successfully found"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
)]
/// Get all aircraft from the database.
pub async fn get_all_aircraft(
    Extension(mut grpc_clients): Extension<GrpcClients>,
) -> Result<Json<Vec<Aircraft>>, (StatusCode, String)> {
    let filter = AdvancedSearchFilter::search_is_not_null(String::from("deleted_at"));
    let vehicle_client_option = grpc_clients.storage_vehicle.get_client().await;
    if vehicle_client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_all_aircraft) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }

    let mut vehicle_client = vehicle_client_option.unwrap();

    let vehicle_response = vehicle_client.search(filter.clone()).await;

    if vehicle_response.is_err() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_all_aircraft) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut vehicles = vehicle_response.unwrap().into_inner().list;

    let mut assets = Vec::new();

    for vehicle in vehicles.drain(..) {
        assets.push(Aircraft::from(vehicle).unwrap());
    }

    Ok(Json(assets))
}

#[utoipa::path(
    post,
    path = "/assets/demo/vertiports",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Assets successfully found"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
)]
/// Get all vertiports from the database.
pub async fn get_all_vertiports(
    Extension(mut grpc_clients): Extension<GrpcClients>,
) -> Result<Json<Vec<Vertiport>>, (StatusCode, String)> {
    let filter = AdvancedSearchFilter::search_is_not_null(String::from("deleted_at"));
    let vertiport_client_option = grpc_clients.storage_vertiport.get_client().await;
    if vertiport_client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_all_vertiports) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }

    let mut vertiport_client = vertiport_client_option.unwrap();

    let vertiport_response = vertiport_client.search(filter.clone()).await;

    if vertiport_response.is_err() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_all_vertiports) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut vertiports = vertiport_response.unwrap().into_inner().list;

    let mut assets = Vec::new();

    for vertiport in vertiports.drain(..) {
        assets.push(Vertiport::from(vertiport).unwrap());
    }

    Ok(Json(assets))
}

#[utoipa::path(
    post,
    path = "/assets/demo/vertipads",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Assets successfully found"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
)]
/// Get all vertipads from the database.
pub async fn get_all_vertipads(
    Extension(mut grpc_clients): Extension<GrpcClients>,
) -> Result<Json<Vec<Vertipad>>, (StatusCode, String)> {
    let filter = AdvancedSearchFilter::search_is_not_null(String::from("deleted_at"));
    let vertipad_client_option = grpc_clients.storage_vertipad.get_client().await;
    if vertipad_client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_all_vertipads) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }

    let mut vertipad_client = vertipad_client_option.unwrap();

    let vertipad_response = vertipad_client.search(filter.clone()).await;

    if vertipad_response.is_err() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_all_vertipads) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut vertipads = vertipad_response.unwrap().into_inner().list;

    let mut assets = Vec::new();

    for vertipad in vertipads.drain(..) {
        assets.push(Vertipad::from(vertipad).unwrap());
    }

    Ok(Json(assets))
}

//-----------------------------------------------------------
// Get assets by operator
//-----------------------------------------------------------
#[utoipa::path(
    get,
    path = "/assets/operators/{id}/assets",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Assets found from database for operator {id}"),
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
    req_debug!("get_all_assets({})", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    // let vertiport_client_option = grpc_clients.storage_vertiport.get_client().await;
    // let vertipad_client_option = grpc_clients.storage_vertipad.get_client().await;
    // if vertiport_client_option.is_none() || vertipad_client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_all_assets) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }

    // let request = tonic::Request::new(SearchFilter {
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
    //         req_error!("(get_all_assets) Error getting vertiports: {}", e);
    //         (
    //             StatusCode::SERVICE_UNAVAILABLE,
    //             "Error getting vertiports".to_string(),
    //         )
    //     })?
    //     .into_inner()
    //     .vertiports;
    //TODO
    Ok(Json(vec![]))
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
        (status = 200, description = "Grouped assets found from database for operator {id}"),
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
    req_debug!("get_all_grouped_assets({})", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_all_grouped_assets) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    //TODO
    Ok(Json(vec![]))
}

/// Get all grouped assets delegated to an operator.
#[utoipa::path(
    get,
    path = "/assets/operators/{id}/grouped/delegated-to",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Grouped assets delegated to operator {id} found from database"),
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
    req_debug!("get_all_grouped_assets_delegated_to({})", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_all_grouped_assets_delegated_to) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    //TODO
    Ok(Json(vec![]))
}

/// Get all grouped assets delegated from an operator.
#[utoipa::path(
    get,
    path = "/assets/operators/{id}/grouped/delegated-from",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Grouped assets delegated from operator {id} found from database"),
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
    req_debug!("get_all_grouped_assets_delegated_from({})", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_all_grouped_assets_delegated_from) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    //TODO
    Ok(Json(vec![]))
}

//-----------------------------------------------------------
// Get assets by asset id
//-----------------------------------------------------------

/// Get an [`Aircraft`] by its id.
#[utoipa::path(
    get,
    path = "/assets/aircraft/{id}",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Aircraft {id} found from database"),
        (status = 404, description = "Aircraft not found in database"),
        (status = 400, description = "Invalid aircraft id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Aircraft id"),
    )
)]
pub async fn get_aircraft_by_id(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(aircraft_id): Path<String>,
) -> Result<Json<Aircraft>, (StatusCode, String)> {
    req_debug!("get_aircraft_by_id({})", aircraft_id);
    if !is_uuid(&aircraft_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid aircraft id".to_string()));
    }

    // Get Client
    let client_option = grpc_clients.storage_vehicle.get_client().await;
    if client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_aircraft_by_id) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut client = client_option.unwrap();

    match client
        .get_by_id(tonic::Request::new(Id {
            id: aircraft_id.clone(),
        }))
        .await
    {
        Ok(response) => {
            let vehicle = response.into_inner();
            let aircraft = Aircraft::from(vehicle);
            if aircraft.is_err() {
                let error_msg = format!(
                    "Error converting storage vehicle to aircraft: {}",
                    aircraft.err().unwrap()
                );
                req_error!("(get_aircraft_by_id) {}", &error_msg);
                return Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg));
            }

            Ok(Json(aircraft.unwrap()))
        }
        Err(e) => {
            let error_msg = format!("Error getting aircraft from storage: {}", e);
            req_error!("(get_aircraft_by_id) {}", &error_msg);
            Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg))
        }
    }
}

/// Get an [`Vertipad`] by its id.
#[utoipa::path(
    get,
    path = "/assets/vertipads/{id}",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Vertipad {id} found from database"),
        (status = 404, description = "Vertipad not found in database"),
        (status = 400, description = "Invalid vertipad id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Vertipad id"),
    )
)]
pub async fn get_vertipad_by_id(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(vertipad_id): Path<String>,
) -> Result<Json<Vertipad>, (StatusCode, String)> {
    req_debug!("get_vertipad_by_id({})", vertipad_id);
    if !is_uuid(&vertipad_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid vertipad id".to_string()));
    }

    // Get Client
    let client_option = grpc_clients.storage_vertipad.get_client().await;
    if client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_vertipad_by_id) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut client = client_option.unwrap();

    match client
        .get_by_id(tonic::Request::new(Id {
            id: vertipad_id.clone(),
        }))
        .await
    {
        Ok(response) => {
            let vertipad = response.into_inner();
            let vertipad = Vertipad::from(vertipad);
            if vertipad.is_err() {
                let error_msg = format!(
                    "Error converting storage vertipad to vertipad: {}",
                    vertipad.err().unwrap()
                );
                req_error!("(get_vertipad_by_id) {}", &error_msg);
                return Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg));
            }

            Ok(Json(vertipad.unwrap()))
        }
        Err(e) => {
            let error_msg = format!("Error getting vertipad from storage: {}", e);
            req_error!("(get_vertipad_by_id) {}", &error_msg);
            Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg))
        }
    }
}

/// Get an [`Vertiport`] by its id.
#[utoipa::path(
    get,
    path = "/assets/vertiports/{id}",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Vertiport {id} found from database"),
        (status = 404, description = "Vertiport not found in database"),
        (status = 400, description = "Invalid vertiport id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Vertiport id"),
    )
)]
pub async fn get_vertiport_by_id(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(vertiport_id): Path<String>,
) -> Result<Json<Vertiport>, (StatusCode, String)> {
    req_debug!("get_vertiport_by_id({})", vertiport_id);
    if !is_uuid(&vertiport_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid vertiport id".to_string()));
    }
    // Get Client
    let client_option = grpc_clients.storage_vertiport.get_client().await;
    if client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_vertiport_by_id) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut client = client_option.unwrap();

    match client
        .get_by_id(tonic::Request::new(Id {
            id: vertiport_id.clone(),
        }))
        .await
    {
        Ok(response) => {
            let vertiport = response.into_inner();
            let vertiport = Vertiport::from(vertiport);
            if vertiport.is_err() {
                let error_msg = format!(
                    "Error converting storage vertiport to vertiport: {}",
                    vertiport.err().unwrap()
                );
                req_error!("(get_vertiport_by_id) {}", &error_msg);
                return Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg));
            }

            Ok(Json(vertiport.unwrap()))
        }
        Err(e) => {
            let error_msg = format!("Error getting vertiport from storage: {}", e);
            req_error!("(get_vertiport_by_id) {}", &error_msg);
            Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg))
        }
    }
}

/// Get an [`AssetGroup`] by its id.
#[utoipa::path(
    get,
    path = "/assets/groups/{id}",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Asset group {id} found from database"),
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
    req_debug!("get_asset_group_by_id({})", asset_group_id);
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
    //     req_error!("(get_asset_group_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    //TODO
    Ok(Json(AssetGroup::random()))
}

//-----------------------------------------------------------
// Register assets
//-----------------------------------------------------------

/// Register an [`Aircraft`] in the database.
#[utoipa::path(
    post,
    path = "/assets/aircraft",
    tag = "svc-assets",
    request_body=RegisterAircraftPayload,
    responses(
        (status = 200, description = "Aircraft registered in database"),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn register_aircraft(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<RegisterAircraftPayload>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("register_aircraft()");

    // Get Client
    let client_option = grpc_clients.storage_vehicle.get_client().await;
    if client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_asset_group_by_id) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut client = client_option.unwrap();

    match client
        .insert(tonic::Request::new(Data {
            last_vertiport_id: None,
            vehicle_model_id: Uuid::new_v4().to_string(),
            serial_number: payload.serial_number,
            registration_number: payload.registration_number,
            description: payload.description,
            asset_group_id: None,
            schedule: None,
            last_maintenance: if let Some(last_maintenance) = payload.last_maintenance {
                let last_maintenance_timezone = Timestamp::from_str(&last_maintenance);
                Some(last_maintenance_timezone.unwrap())
            } else {
                None
            },
            next_maintenance: if let Some(next_maintenance) = payload.next_maintenance {
                let next_maintenance_timezone = Timestamp::from_str(&next_maintenance);
                Some(next_maintenance_timezone.unwrap())
            } else {
                None
            },
        }))
        .await
    {
        Ok(res) => {
            println!("RESPONSE Vehicle Insert={:?}", res);
            let vehicle_obj = res.into_inner().object;
            if let Some(vehicle_obj) = vehicle_obj {
                Ok(vehicle_obj.id)
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Could not insert vehicle".to_string(),
                ))
            }
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Register an [`Vertiport`] in the database.
#[utoipa::path(
    post,
    path = "/assets/vertiports",
    tag = "svc-assets",
    request_body=RegisterVertiportPayload,
    responses(
        (status = 200, description = "Vertiport registered in database"),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn register_vertiport(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<RegisterVertiportPayload>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("register_vertiport()");

    // Get Client
    let client_option = grpc_clients.storage_vertiport.get_client().await;
    if client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_asset_group_by_id) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut client = client_option.unwrap();

    match client
        .insert(tonic::Request::new(VertiportData {
            name: payload.name.unwrap_or_else(|| "unnamed".to_string()),
            description: payload.description.unwrap_or_else(|| "N/A".to_string()),
            latitude: payload.location.latitude.to_f64(),
            longitude: payload.location.longitude.to_f64(),
            schedule: None,
        }))
        .await
    {
        Ok(res) => {
            println!("RESPONSE Vertiport Insert={:?}", res);
            let vertiport_obj = res.into_inner().object;
            if let Some(vertiport_obj) = vertiport_obj {
                Ok(vertiport_obj.id)
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Could not insert vertiport".to_string(),
                ))
            }
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Register an [`Vertipad`] in the database.
///
/// Also inserts the vertipad into the vertiport's vertipad list.
#[utoipa::path(
    post,
    path = "/assets/vertipads",
    tag = "svc-assets",
    request_body=RegisterVertipadPayload,
    responses(
        (status = 200, description = "Vertipad registered in database"),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn register_vertipad(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<RegisterVertipadPayload>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("register_vertipad()");

    // Get Client
    let client_option = grpc_clients.storage_vertipad.get_client().await;
    if client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_asset_group_by_id) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut client = client_option.unwrap();

    match client
        .insert(tonic::Request::new(VertipadData {
            name: payload.name.unwrap_or_else(|| "unnamed".to_string()),
            vertiport_id: payload.vertiport_id.clone(),
            latitude: payload.location.latitude.to_f64(),
            longitude: payload.location.longitude.to_f64(),
            schedule: None,
            enabled: payload.enabled,
            occupied: payload.occupied,
        }))
        .await
    {
        Ok(res) => {
            println!("RESPONSE Vertipad Insert={:?}", res);
            let vertipad_obj = res.into_inner().object;
            if let Some(vertipad_obj) = vertipad_obj {
                Ok(vertipad_obj.id)
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Could not insert vertipad".to_string(),
                ))
            }
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

//-----------------------------------------------------------
// Group management
//-----------------------------------------------------------

/// Register an [`AssetGroup`] in the database.
#[utoipa::path(
    post,
    path = "/assets/groups",
    tag = "svc-assets",
    request_body=RegisterAssetGroupPayload,
    responses(
        (status = 200, description = "AssetGroup registered in database"),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn register_asset_group(
    Extension(mut _grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<RegisterAssetGroupPayload>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("register_asset_group()");

    // validate payload
    // to check with the database to validate the registration number
    //
    // if !payload.is_valid() {
    //     return Err((
    // StatusCode::BAD_REQUEST,
    //         "Invalid payload format".to_string(),
    //     ));
    // }

    let _asset_group = AssetGroup {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        owner: payload.owner,
        created_at: Utc::now(),
        updated_at: None,
        delegatee: None,
        assets: payload.assets,
    };

    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_asset_group_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    //TODO
    Ok(_asset_group.id)
}

//-----------------------------------------------------------
// Asset Updates
//-----------------------------------------------------------

/// Update/modify an [`Aircraft`] in the database.
///
/// This will update the aircraft's information.
#[utoipa::path(
    put,
    path = "/assets/aircraft",
    tag = "svc-assets",
    request_body=Aircraft,
    responses(
        (status = 200, description = "Aircraft updated in database"),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn update_aircraft(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<UpdateAircraftPayload>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("update_aircraft()");

    let vehicle_id = payload.id.clone();
    // Get Client
    let client_option = grpc_clients.storage_vehicle.get_client().await;
    if client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_asset_group_by_id) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut client = client_option.unwrap();

    let response = match client
        .get_by_id(tonic::Request::new(Id {
            id: vehicle_id.clone(),
        }))
        .await
    {
        Ok(res) => {
            println!("RESPONSE Vehicle By ID={:?}", res);
            res
        }
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    let vehicle = response.into_inner().data.unwrap();

    match client
        .update(tonic::Request::new(UpdateObject {
            id: vehicle_id.clone(),
            data: Some(Data {
                last_vertiport_id: payload
                    .last_vertiport_id
                    .unwrap_or(vehicle.last_vertiport_id),
                vehicle_model_id: payload.vehicle_model_id.unwrap_or(vehicle.vehicle_model_id),
                serial_number: payload.serial_number.unwrap_or(vehicle.serial_number),
                registration_number: payload
                    .registration_number
                    .unwrap_or(vehicle.registration_number),
                description: payload.description.unwrap_or(vehicle.description),
                asset_group_id: payload.asset_group_id.unwrap_or(vehicle.asset_group_id),
                schedule: payload.schedule.unwrap_or(vehicle.schedule),
                last_maintenance: if let Some(last_maintenance) = payload.last_maintenance {
                    if let Some(last_maintenance) = last_maintenance {
                        let time_stamp_result = Timestamp::from_str(&last_maintenance);
                        if time_stamp_result.is_ok() {
                            Some(time_stamp_result.unwrap())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    vehicle.last_maintenance
                },

                next_maintenance: if let Some(next_maintenance) = payload.next_maintenance {
                    if let Some(next_maintenance) = next_maintenance {
                        let time_stamp_result = Timestamp::from_str(&next_maintenance);
                        if time_stamp_result.is_ok() {
                            Some(time_stamp_result.unwrap())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    vehicle.next_maintenance
                },
            }),
            mask: Some(FieldMask {
                paths: payload.mask,
            }),
        }))
        .await
    {
        Ok(res) => {
            println!("RESPONSE Vehicle Update={:?}", res);
            Ok(vehicle_id.clone())
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Update/modify a [`Vertiport`] in the database.
///
/// This will update the vertiport's information. It can also be used to
/// perform batch add/remove of vertipads.
#[utoipa::path(
    put,
    path = "/assets/vertiports",
    tag = "svc-assets",
    request_body=Vertiport,
    responses(
        (status = 200, description = "Vertiport updated in database"),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn update_vertiport(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<UpdateVertiportPayload>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("update_vertiport()");

    // Get Client
    let client_option = grpc_clients.storage_vertiport.get_client().await;
    if client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_asset_group_by_id) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut client = client_option.unwrap();

    let response = match client
        .get_by_id(tonic::Request::new(Id {
            id: payload.id.clone(),
        }))
        .await
    {
        Ok(res) => {
            println!("RESPONSE Vertiport By ID={:?}", res);
            res
        }
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    let vertiport = response.into_inner().data.unwrap();

    match client
        .update(tonic::Request::new(VertiportUpdateObject {
            id: payload.id.clone(),
            data: Some(VertiportData {
                name: payload.name.unwrap_or(vertiport.name),
                description: payload.description.unwrap_or(vertiport.description),
                latitude: payload.latitude.unwrap_or(vertiport.latitude),
                longitude: payload.longitude.unwrap_or(vertiport.longitude),
                schedule: payload.schedule.unwrap_or(vertiport.schedule),
            }),
            mask: Some(FieldMask {
                paths: payload.mask,
            }),
        }))
        .await
    {
        Ok(res) => {
            println!("RESPONSE Vertiport Update={:?}", res);
            Ok(payload.id.clone())
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Update/modify a [`Vertipad`] in the database.
#[utoipa::path(
    put,
    path = "/assets/vertipads",
    tag = "svc-assets",
    request_body=Vertipad,
    responses(
        (status = 200, description = "Vertipad updated in database"),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn update_vertipad(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<UpdateVertipadPayload>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("update_vertipad()");
    // Get Client
    let client_option = grpc_clients.storage_vertipad.get_client().await;
    if client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_asset_group_by_id) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut client = client_option.unwrap();

    let response = match client
        .get_by_id(tonic::Request::new(Id {
            id: payload.id.clone(),
        }))
        .await
    {
        Ok(res) => {
            println!("RESPONSE Vertipad By ID={:?}", res);
            res
        }
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    let vertipad = response.into_inner().data.unwrap();

    match client
        .update(tonic::Request::new(VertipadUpdateObject {
            id: payload.id.clone(),
            data: Some(VertipadData {
                name: payload.name.unwrap_or(vertipad.name),
                latitude: payload.latitude.unwrap_or(vertipad.latitude),
                longitude: payload.longitude.unwrap_or(vertipad.longitude),
                enabled: payload.enabled.unwrap_or(vertipad.enabled),
                occupied: payload.occupied.unwrap_or(vertipad.occupied),
                schedule: payload.schedule.unwrap_or(vertipad.schedule),
                vertiport_id: payload.vertiport_id.unwrap_or(vertipad.vertiport_id),
            }),
            mask: Some(FieldMask {
                paths: payload.mask,
            }),
        }))
        .await
    {
        Ok(res) => {
            println!("RESPONSE Vertipad Update={:?}", res);
            Ok(payload.id.clone())
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Update/modify an [`AssetGroup`] in the database.
#[utoipa::path(
    put,
    path = "/assets/groups/{id}",
    tag = "svc-assets",
    request_body=AssetGroup,
    responses(
        (status = 200, description = "AssetGroup updated in database"),
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
    req_debug!("update_asset_group()");
    // TODO: validate payload - need to check ownerships, existence, etc.

    // validate payload
    // to check with the database to validate the registration number
    //
    // if !payload.is_valid() {
    //     return Err((
    // StatusCode::BAD_REQUEST,
    //         "Invalid payload format".to_string(),
    //     ));
    // }

    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_asset_group_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    //TODO
    Ok(payload.id)
}

//-----------------------------------------------------------
// Asset Deletion
//-----------------------------------------------------------

/// Remove a [`Aircraft`] from the database.
#[utoipa::path(
    delete,
    path = "/assets/aircraft/{id}",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Aircraft removed from database"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Aircraft id"),
    )
)]
pub async fn remove_aircraft(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(id): Path<String>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("remove_aircraft()");

    // Get Client
    let client_option = grpc_clients.storage_vehicle.get_client().await;
    if client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_asset_group_by_id) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut client = client_option.unwrap();

    match client
        .delete(tonic::Request::new(Id { id: id.clone() }))
        .await
    {
        Ok(res) => {
            println!("RESPONSE Vertipad Delete={:?}", res);
            Ok(id)
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Remove a [`Vertipad`] from the database.
#[utoipa::path(
    delete,
    path = "/assets/vertipads/{id}",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Vertipad removed from database"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Vertipad id"),
    )
)]
pub async fn remove_vertipad(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(id): Path<String>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("remove_vertipad()");

    // Get Client
    let client_option = grpc_clients.storage_vertipad.get_client().await;
    if client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_asset_group_by_id) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut client = client_option.unwrap();

    match client
        .delete(tonic::Request::new(Id { id: id.clone() }))
        .await
    {
        Ok(res) => {
            println!("RESPONSE Vertipad Delete={:?}", res);
            Ok(id)
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Remove a [`Vertiport`] from the database.
#[utoipa::path(
    delete,
    path = "/assets/vertiports/{id}",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Vertiport removed from database"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Vertiport id"),
    )
)]
pub async fn remove_vertiport(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(id): Path<String>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("remove_vertiport()");

    // Get Client
    let client_option = grpc_clients.storage_vertiport.get_client().await;
    if client_option.is_none() {
        let error_msg = "svc-storage unavailable.".to_string();
        req_error!("(get_asset_group_by_id) {}", &error_msg);
        return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    }
    let mut client = client_option.unwrap();

    match client
        .delete(tonic::Request::new(Id { id: id.clone() }))
        .await
    {
        Ok(res) => {
            println!("RESPONSE Vertiport Delete={:?}", res);
            Ok(id)
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Remove an [`AssetGroup`] from the database.
#[utoipa::path(
    delete,
    path = "/assets/groups/{id}",
    tag = "svc-assets",
    responses(
        (status = 200, description = "AssetGroup removed from database"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "AssetGroup id"),
    )
)]
pub async fn remove_asset_group(
    Extension(mut _grpc_clients): Extension<GrpcClients>,
    Path(_id): Path<String>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("remove_asset_group()");
    // TODO: validate payload - need to check ownerships, existence,
    // etc.
    // TODO: set associated assets' 'group_id' to None

    // validate payload
    // to check with the database to validate the registration number
    //
    // if !payload.is_valid() {
    //     return Err((
    // StatusCode::BAD_REQUEST,
    //         "Invalid payload format".to_string(),
    //     ));
    // }

    // Get Client
    // let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_asset_group_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    //TODO
    Ok(_id)
}
