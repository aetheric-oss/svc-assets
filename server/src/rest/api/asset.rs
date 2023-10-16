//! Implementation of asset-related REST APIs.

pub use super::rest_types::*;
use prost_types::FieldMask;

use axum::{extract::Path, Extension, Json};
use hyper::StatusCode;
// use lib_common::grpc::ClientConnect;
use svc_storage_client_grpc::prelude::*;

use super::is_uuid;
use super::structs::{Aircraft, Vertipad, Vertiport};
use crate::grpc::client::GrpcClients;
use uuid::Uuid;

//-----------------------------------------------------------
// Asset Deletion
//-----------------------------------------------------------

/// Remove a [`Aircraft`] from the database.
#[utoipa::path(
    delete,
    path = "/assets/aircraft/{id}",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Aircraft removed from database; a UUID is returned", body = String),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Aircraft id"),
    )
)]
pub async fn remove_aircraft(
    Extension(grpc_clients): Extension<GrpcClients>,
    Path(id): Path<String>,
) -> Result<String, (StatusCode, String)> {
    rest_info!("(remove_aircraft) entry [{}].", &id);

    let client = grpc_clients.storage.vehicle;

    match client.delete(Id { id: id.clone() }).await {
        Ok(res) => {
            rest_info!("(remove_aircraft) successfully removed aircraft {:?}", res);
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
        (status = 200, description = "Vertipad removed from database; a UUID is returned", body = String),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Vertipad id"),
    )
)]
pub async fn remove_vertipad(
    Extension(grpc_clients): Extension<GrpcClients>,
    Path(id): Path<String>,
) -> Result<String, (StatusCode, String)> {
    rest_info!("(remove_vertipad) entry [{}].", &id);

    let client = grpc_clients.storage.vertipad;

    match client.delete(Id { id: id.clone() }).await {
        Ok(res) => {
            rest_info!("(remove_vertipad) successfully removed vertipad {:?}", res);
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
        (status = 200, description = "Vertiport removed from database; a UUID is returned", body = String),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Vertiport id"),
    )
)]
pub async fn remove_vertiport(
    Extension(grpc_clients): Extension<GrpcClients>,
    Path(id): Path<String>,
) -> Result<String, (StatusCode, String)> {
    rest_info!("(remove_vertiport) entry [{}].", &id);

    let client = grpc_clients.storage.vertiport;

    match client.delete(Id { id: id.clone() }).await {
        Ok(res) => {
            rest_info!(
                "(remove_vertiport) successfully removed vertiport {:?}",
                res
            );
            Ok(id)
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
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
    request_body=UpdateAircraftPayload,
    responses(
        (status = 200, description = "Aircraft updated in database; a UUID is returned", body = String),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn update_aircraft(
    Extension(grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<UpdateAircraftPayload>,
) -> Result<String, (StatusCode, String)> {
    rest_info!("(update_aircraft) entry [{}].", payload.id);
    rest_debug!("(update_aircraft) Payload: {:?}", &payload);

    let vehicle_id = payload.id.clone();
    let client = grpc_clients.storage.vehicle;

    let response = match client
        .get_by_id(Id {
            id: vehicle_id.clone(),
        })
        .await
    {
        Ok(res) => {
            rest_debug!("(update_aircraft) successfully got vehicle {:?}", res);
            res
        }
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    let vehicle = match response.into_inner().data {
        Some(data) => data,
        None => {
            return Err((StatusCode::NOT_FOUND, "Vehicle not found".to_string()));
        }
    };

    // let last_maintenance = match payload.last_maintenance {
    //     Some(last_maintenance) => match Timestamp::from_str(&last_maintenance) {
    //         Ok(time_stamp) => Some(time_stamp),
    //         Err(e) => {
    //             rest_error!("(update_aircraft) {}", &e.to_string());
    //             return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    //         }
    //     }
    //     None => vehicle.last_maintenance
    // };

    // let next_maintenance = match payload.next_maintenance {
    //     Some(next_maintenance) => match Timestamp::from_str(&next_maintenance) {
    //         Ok(time_stamp) => Some(time_stamp),
    //         Err(e) => {
    //             rest_error!("(update_aircraft) {}", &e.to_string());
    //             return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    //         }
    //     }
    //     None => vehicle.next_maintenance
    // };

    let data = vehicle::Data {
        last_vertiport_id: payload.last_vertiport_id,
        vehicle_model_id: payload.vehicle_model_id.unwrap_or(vehicle.vehicle_model_id),
        serial_number: payload.serial_number.unwrap_or(vehicle.serial_number),
        registration_number: payload
            .registration_number
            .unwrap_or(vehicle.registration_number),
        description: payload.description.unwrap_or(vehicle.description),
        asset_group_id: payload.asset_group_id.unwrap_or(vehicle.asset_group_id),
        schedule: payload.schedule.unwrap_or(vehicle.schedule),
        last_maintenance: None,
        next_maintenance: None,
        created_at: None,
        updated_at: None,
    };

    match client
        .update(vehicle::UpdateObject {
            id: vehicle_id.clone(),
            data: Some(data),
            mask: Some(FieldMask {
                paths: payload.mask,
            }),
        })
        .await
    {
        Ok(res) => {
            rest_info!("(update_aircraft) successfully updated vehicle {:?}", res);
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
    request_body=UpdateVertiportPayload,
    responses(
        (status = 200, description = "Vertiport updated in database; a UUID is returned", body = String),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn update_vertiport(
    Extension(grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<UpdateVertiportPayload>,
) -> Result<String, (StatusCode, String)> {
    rest_info!("(update_vertiport) entry [{}].", payload.id);
    rest_debug!("(update_vertiport) Payload: {:?}", &payload);

    let client = grpc_clients.storage.vertiport;

    let response = match client
        .get_by_id(Id {
            id: payload.id.clone(),
        })
        .await
    {
        Ok(res) => {
            rest_info!("(update_vertiport) successfully got vertiport {:?}", res);
            res
        }
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    let vertiport = match response.into_inner().data {
        Some(data) => data,
        None => {
            return Err((StatusCode::NOT_FOUND, "Vertiport not found".to_string()));
        }
    };

    match client
        .update(vertiport::UpdateObject {
            id: payload.id.clone(),
            data: Some(vertiport::Data {
                name: payload.name.unwrap_or(vertiport.name),
                description: payload.description.unwrap_or(vertiport.description),
                geo_location: payload.geo_location.or(vertiport.geo_location),
                schedule: payload.schedule.unwrap_or(vertiport.schedule),
                created_at: None,
                updated_at: None,
            }),
            mask: Some(FieldMask {
                paths: payload.mask,
            }),
        })
        .await
    {
        Ok(res) => {
            rest_info!(
                "(update_vertiport) successfully updated vertiport {:?}",
                res
            );
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
    request_body=UpdateVertipadPayload,
    responses(
        (status = 200, description = "Vertipad updated in database; a UUID is returned", body = String),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn update_vertipad(
    Extension(grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<UpdateVertipadPayload>,
) -> Result<String, (StatusCode, String)> {
    rest_info!("(update_vertipad) entry [{}].", payload.id);
    rest_debug!("(update_vertipad) Payload: {:?}", &payload);

    let client = grpc_clients.storage.vertipad;

    let response = match client
        .get_by_id(Id {
            id: payload.id.clone(),
        })
        .await
    {
        Ok(res) => {
            rest_info!("(update_vertipad) successfully got vertipad {:?}", res);
            res
        }
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    let vertipad = match response.into_inner().data {
        Some(data) => data,
        None => {
            return Err((StatusCode::NOT_FOUND, "Vertipad not found".to_string()));
        }
    };

    match client
        .update(vertipad::UpdateObject {
            id: payload.id.clone(),
            data: Some(vertipad::Data {
                name: payload.name.unwrap_or(vertipad.name),
                geo_location: payload.geo_location.or(vertipad.geo_location),
                enabled: payload.enabled.unwrap_or(vertipad.enabled),
                occupied: payload.occupied.unwrap_or(vertipad.occupied),
                schedule: payload.schedule.unwrap_or(vertipad.schedule),
                vertiport_id: payload.vertiport_id.unwrap_or(vertipad.vertiport_id),
                created_at: None,
                updated_at: None,
            }),
            mask: Some(FieldMask {
                paths: payload.mask,
            }),
        })
        .await
    {
        Ok(res) => {
            rest_info!("(update_vertipad) successfully updated vertipad {:?}", res);
            Ok(payload.id.clone())
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

//-----------------------------------------------------------
// Register assets
//-----------------------------------------------------------

/// Register an [`Aircraft`] in the database.
#[utoipa::path(
    post,
    path = "/assets/aircraft",
    tag = "svc-assets",
    request_body=vehicle::Data,
    responses(
        (status = 200, description = "Aircraft registered in database; a UUID is returned", body = String),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn register_aircraft(
    Extension(grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<vehicle::Data>,
) -> Result<String, (StatusCode, String)> {
    rest_info!("(register_aircraft) entry.");
    rest_debug!("(register_aircraft) Payload: {:?}", &payload);

    let client = grpc_clients.storage.vehicle;

    let data = vehicle::Data {
        last_vertiport_id: payload.last_vertiport_id,
        vehicle_model_id: Uuid::new_v4().to_string(),
        serial_number: payload.serial_number,
        registration_number: payload.registration_number,
        description: payload.description,
        asset_group_id: None,
        schedule: None,
        last_maintenance: payload.last_maintenance,
        next_maintenance: payload.next_maintenance,
        created_at: None,
        updated_at: None,
    };

    match client.insert(data).await {
        Ok(res) => {
            rest_info!("(register_aircraft) registration success.");
            rest_debug!("(register_aircraft) {:?}", res);
            let vehicle_obj = res.into_inner().object;
            if let Some(vehicle_obj) = vehicle_obj {
                rest_debug!(
                    "(register_aircraft) got new aircraft id: {}",
                    vehicle_obj.id
                );
                Ok(vehicle_obj.id)
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "(register_aircraft) could not insert vehicle.".to_string(),
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
    request_body=vertiport::Data,
    responses(
        (status = 200, description = "Vertiport registered in database; a UUID is returned", body = String),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn register_vertiport(
    Extension(grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<vertiport::Data>,
) -> Result<String, (StatusCode, String)> {
    rest_info!("(register_vertiport) entry.");
    rest_debug!("(register_vertiport) Payload: {:?}", &payload);

    let client = grpc_clients.storage.vertiport;

    match client.insert(payload).await {
        Ok(res) => {
            rest_info!("(register_vertiport) registration success.");
            rest_debug!("(register_vertiport) {:?}", res);
            let vertiport_obj = res.into_inner().object;
            if let Some(vertiport_obj) = vertiport_obj {
                rest_debug!(
                    "(register_vertiport) got new vertiport id: {}",
                    vertiport_obj.id
                );
                Ok(vertiport_obj.id)
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "(register_vertiport) could not insert vertiport.".to_string(),
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
    request_body=vertipad::Data,
    responses(
        (status = 200, description = "Vertipad registered in database; a UUID is returned", body = String),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn register_vertipad(
    Extension(grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<vertipad::Data>,
) -> Result<String, (StatusCode, String)> {
    rest_info!("(register_vertipad) entry.");
    rest_debug!("(register_vertipad) Payload: {:?}", &payload);

    let client = grpc_clients.storage.vertipad;

    match client.insert(payload).await {
        Ok(res) => {
            rest_info!("(register_vertipad) registration success.");
            rest_debug!("(register_vertipad) {:?}", res);
            let vertipad_obj = res.into_inner().object;
            if let Some(vertipad_obj) = vertipad_obj {
                rest_debug!(
                    "(register_vertipad) got new vertipad id: {}",
                    vertipad_obj.id
                );
                Ok(vertipad_obj.id)
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "(register_vertipad) could not insert vertipad.".to_string(),
                ))
            }
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
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
        (status = 200, description = "Aircraft {id} found from database", body = Aircraft),
        (status = 404, description = "Aircraft not found in database"),
        (status = 400, description = "Invalid aircraft id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Aircraft id"),
    )
)]
pub async fn get_aircraft_by_id(
    Extension(grpc_clients): Extension<GrpcClients>,
    Path(aircraft_id): Path<String>,
) -> Result<Json<Aircraft>, (StatusCode, String)> {
    rest_info!("(get_aircraft_by_id) entry [{}].", aircraft_id);
    if !is_uuid(&aircraft_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid aircraft id".to_string()));
    }

    let client = grpc_clients.storage.vehicle;
    match client
        .get_by_id(Id {
            id: aircraft_id.clone(),
        })
        .await
    {
        Ok(response) => {
            let vehicle = response.into_inner();
            let aircraft: Aircraft = match vehicle.try_into() {
                Ok(aircraft) => {
                    rest_info!("(get_aircraft_by_id) Aircraft found: {}", aircraft_id);
                    aircraft
                }
                Err(e) => {
                    let error_msg = format!("Error converting vehicle to aircraft: {}", e);
                    rest_error!("(get_aircraft_by_id) {}", &error_msg);
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg));
                }
            };
            Ok(Json(aircraft))
        }
        Err(e) => {
            let error_msg = format!("Error getting aircraft from storage: {}", e);
            rest_error!("(get_aircraft_by_id) {}", &error_msg);
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
        (status = 200, description = "Vertipad {id} found from database", body = Vertipad),
        (status = 404, description = "Vertipad not found in database"),
        (status = 400, description = "Invalid vertipad id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Vertipad id"),
    )
)]
pub async fn get_vertipad_by_id(
    Extension(grpc_clients): Extension<GrpcClients>,
    Path(vertipad_id): Path<String>,
) -> Result<Json<Vertipad>, (StatusCode, String)> {
    rest_info!("(get_vertipad_by_id) entry [{}].", vertipad_id);
    if !is_uuid(&vertipad_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid vertipad id".to_string()));
    }

    let client = grpc_clients.storage.vertipad;
    match client
        .get_by_id(Id {
            id: vertipad_id.clone(),
        })
        .await
    {
        Ok(response) => {
            let vertipad = response.into_inner();
            let vertipad: Vertipad = match vertipad.try_into() {
                Ok(vertipad) => {
                    rest_info!("(get_vertipad_by_id) Vertipad found: {}", vertipad_id);
                    vertipad
                }
                Err(e) => {
                    let error_msg = format!("Error converting vehicle to vertipad: {}", e);
                    rest_error!("(get_vertipad_by_id) {}", &error_msg);
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg));
                }
            };
            Ok(Json(vertipad))
        }
        Err(e) => {
            let error_msg = format!("Error getting vertipad from storage: {}", e);
            rest_error!("(get_vertipad_by_id) {}", &error_msg);
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
        (status = 200, description = "Vertiport {id} found from database", body = Vertiport),
        (status = 404, description = "Vertiport not found in database"),
        (status = 400, description = "Invalid vertiport id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Vertiport id"),
    )
)]
pub async fn get_vertiport_by_id(
    Extension(grpc_clients): Extension<GrpcClients>,
    Path(vertiport_id): Path<String>,
) -> Result<Json<Vertiport>, (StatusCode, String)> {
    rest_info!("(get_vertiport_by_id) entry [{}].", vertiport_id);
    if !is_uuid(&vertiport_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid vertiport id".to_string()));
    }

    let client = grpc_clients.storage.vertiport;
    match client
        .get_by_id(Id {
            id: vertiport_id.clone(),
        })
        .await
    {
        Ok(response) => {
            let vertiport = response.into_inner();
            let vertiport: Vertiport = match vertiport.try_into() {
                Ok(vertiport) => {
                    rest_info!("(get_vertiport_by_id) Vertiport found: {}", vertiport_id);
                    vertiport
                }
                Err(e) => {
                    let error_msg = format!("Error converting vehicle to vertiport: {}", e);
                    rest_error!("(get_vertiport_by_id) {}", &error_msg);
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg));
                }
            };
            Ok(Json(vertiport))
        }
        Err(e) => {
            let error_msg = format!("Error getting vertiport from storage: {}", e);
            rest_error!("(get_vertiport_by_id) {}", &error_msg);
            Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg))
        }
    }
}

#[utoipa::path(
    get,
    path = "/assets/demo/aircraft",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Assets successfully found", body = [Aircraft]),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
)]

/// Get all aircraft from the database.
pub async fn get_all_aircraft(
    Extension(grpc_clients): Extension<GrpcClients>,
) -> Result<Json<Vec<Aircraft>>, (StatusCode, String)> {
    rest_info!("(get_all_aircraft) entry.");
    let filter = AdvancedSearchFilter::search_is_not_null(String::from("deleted_at"));

    let vehicle_client = grpc_clients.storage.vehicle;
    let mut vehicles = match vehicle_client.search(filter.clone()).await {
        Ok(response) => response.into_inner().list,
        Err(e) => {
            let error_msg = "could not retrieve vehicles.".to_string();
            rest_error!("(get_all_aircraft) {}: {}.", error_msg, e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg));
        }
    };

    let mut assets = Vec::new();

    for vehicle in vehicles.drain(..) {
        let aircraft: Aircraft = match vehicle.try_into() {
            Ok(object) => object,
            Err(_) => {
                let error_msg = "could not convert VehicleObject to Aircraft.".to_string();
                rest_error!("(get_all_aircraft) {}", &error_msg);
                return Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg));
            }
        };
        assets.push(aircraft);
    }

    Ok(Json(assets))
}

#[utoipa::path(
    get,
    path = "/assets/demo/vertiports",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Assets successfully found", body = [Vertiport]),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
)]
/// Get all vertiports from the database.
pub async fn get_all_vertiports(
    Extension(grpc_clients): Extension<GrpcClients>,
) -> Result<Json<Vec<Vertiport>>, (StatusCode, String)> {
    rest_info!("(get_all_vertiports) entry.");
    let filter = AdvancedSearchFilter::search_is_not_null(String::from("deleted_at"));

    let vertiport_client = grpc_clients.storage.vertiport;
    let mut vertiports = match vertiport_client.search(filter.clone()).await {
        Ok(response) => response.into_inner().list,
        Err(e) => {
            let error_msg = "could not retrieve vertiports.".to_string();
            rest_error!("(get_all_vertiports) {}: {}.", error_msg, e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg));
        }
    };

    let mut assets = Vec::new();

    for vertiport in vertiports.drain(..) {
        let vertiport: Vertiport = match vertiport.try_into() {
            Ok(object) => object,
            Err(_) => {
                let error_msg = "could not convert VertiportObject to Vertiport.".to_string();
                rest_error!("(get_all_vertiports) {}", &error_msg);
                return Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg));
            }
        };
        assets.push(vertiport);
    }

    Ok(Json(assets))
}

#[utoipa::path(
    get,
    path = "/assets/demo/vertipads",
    tag = "svc-assets",
    responses(
        (status = 200, description = "Assets successfully found", body = [Vertipad]),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
)]
/// Get all vertipads from the database.
pub async fn get_all_vertipads(
    Extension(grpc_clients): Extension<GrpcClients>,
) -> Result<Json<Vec<Vertipad>>, (StatusCode, String)> {
    rest_info!("(get_all_vertipads) entry.");
    let filter = AdvancedSearchFilter::search_is_not_null(String::from("deleted_at"));

    let vertipad_client = grpc_clients.storage.vertipad;
    let mut vertipads = match vertipad_client.search(filter.clone()).await {
        Ok(response) => response.into_inner().list,
        Err(e) => {
            let error_msg = "could not retrieve vertipads.".to_string();
            rest_error!("(get_all_vertipads) {}: {}.", error_msg, e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg));
        }
    };

    let mut assets = Vec::new();

    for vertipad in vertipads.drain(..) {
        let vertipad: Vertipad = match vertipad.try_into() {
            Ok(object) => object,
            Err(_) => {
                let error_msg = "could not convert VertipadObject to Vertipad.".to_string();
                rest_error!("(get_all_vertipads) {}", &error_msg);
                return Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg));
            }
        };
        assets.push(vertipad);
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

    let _filter =
        AdvancedSearchFilter::search_equals(String::from("operator_id"), operator_id.clone());

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
