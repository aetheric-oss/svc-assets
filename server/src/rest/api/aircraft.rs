//! Handlers for actions on aircraft assets

pub use super::rest_types::UpdateAircraftPayload;
use crate::grpc::client::GrpcClients;
use crate::rest::structs::{AssetStatus, Basics};
use axum::{extract::Path, Extension, Json};
use hyper::StatusCode;
use lib_common::time::{DateTime, Utc};
use lib_common::uuid::to_uuid;
use svc_storage_client_grpc::prelude::vehicle;
use svc_storage_client_grpc::prelude::*;

/// A struct representing an aircraft.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Aircraft {
    /// Common information for an asset
    pub basics: Basics,

    /// The aircraft's manufacturer.
    ///
    /// TODO(R4): For now we can just say "Airbus", etc. Later, we
    /// can a struct for this and store the manufacturer's name, logo,
    /// etc.
    pub manufacturer: String,

    /// The aircraft's model.
    pub model: String,

    /// The aircraft's serial number.
    pub serial_number: String,

    /// The aircraft's registration number.
    ///
    /// In the US, this is the N number.
    ///
    /// This is a unique identifier for
    /// the aircraft that can be used to look up information about the
    /// aircraft from national aviation authorities like the FAA.
    pub registration_number: String,

    /// Optional additional description of the aircraft.
    pub description: Option<String>,

    /// The maximum payload the aircraft can carry in kilograms.
    pub max_payload_kg: f64,

    /// The maximum range the aircraft can fly in kilometers.
    pub max_range_km: f64,

    /// The date of the aircraft's last maintenance.
    pub last_maintenance: Option<DateTime<Utc>>,

    /// The date of the aircraft's next planned maintenance.
    pub next_maintenance: Option<DateTime<Utc>>,

    /// The hangar ID where the aircraft is stored.
    pub hangar_id: Option<String>,
}

impl Aircraft {
    /// Get the aircraft's name.
    ///
    /// If the aircraft does not have a name, this will be `manufacturer
    /// model registration_number`.
    ///
    /// For example, "Airbus 737-800 N12345".
    pub fn full_name(&self) -> String {
        match &self.basics.name {
            Some(name) => name.clone(),
            None => format!(
                "{} {} {}",
                self.manufacturer, self.model, self.registration_number
            ),
        }
    }
}

impl TryFrom<vehicle::Object> for Aircraft {
    type Error = String;

    fn try_from(object: vehicle::Object) -> Result<Self, Self::Error> {
        let data = match object.data {
            Some(data) => data,
            None => return Err("(try_from) vehicle data is missing".to_string()),
        };

        Ok(Aircraft {
            basics: Basics {
                id: object.id,
                group_id: data.asset_group_id,
                name: None,
                created_at: data
                    .created_at
                    .ok_or("(try_from) created_at field empty but expected.")?
                    .into(),
                updated_at: data
                    .updated_at
                    .ok_or("(try_from) updated_at field empty but expected.")?
                    .into(),
                whitelist: Vec::new(),
                status: AssetStatus::Available,
            },
            manufacturer: "Arrow".to_string(),
            model: data.vehicle_model_id,
            serial_number: data.serial_number,
            registration_number: data.registration_number,
            description: data.description,
            max_payload_kg: 0.0,
            max_range_km: 0.0,
            last_maintenance: data.last_maintenance.map(|date| date.into()),
            next_maintenance: data.next_maintenance.map(|date| date.into()),
            hangar_id: data.hangar_id,
        })
    }
}

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
) -> Result<(), StatusCode> {
    rest_info!("(update_aircraft) entry [{}].", payload.id);
    rest_debug!("(update_aircraft) Payload: {:?}", &payload);

    let id = to_uuid(&payload.id)
        .ok_or_else(|| {
            rest_error!("(update_aircraft) Invalid aircraft id.");
            StatusCode::BAD_REQUEST
        })? // Check if the aircraft_id is a valid UUID
        .to_string();

    let mut vehicle_data = grpc_clients
        .storage
        .vehicle
        .get_by_id(Id { id: id.clone() })
        .await
        .map_err(|e| {
            rest_error!("(update_aircraft) could not retrieve vehicles: {e}.");
            StatusCode::NOT_FOUND
        })?
        .into_inner()
        .data
        .ok_or_else(|| {
            rest_error!("(update_aircraft) vehicle data is missing.");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    vehicle_data.hangar_id = payload.hangar_id;
    vehicle_data.hangar_bay_id = payload.hangar_bay_id;
    vehicle_data.description = payload.description;
    vehicle_data.asset_group_id = payload.asset_group_id;
    vehicle_data.schedule = payload.schedule;

    if let Some(id) = payload.vehicle_model_id {
        vehicle_data.vehicle_model_id = id;
    }

    if let Some(serial_number) = payload.serial_number {
        vehicle_data.serial_number = serial_number;
    }

    if let Some(registration_number) = payload.registration_number {
        vehicle_data.registration_number = registration_number;
    }

    if let Some(date) = payload.last_maintenance {
        vehicle_data.last_maintenance = Some(date.into());
    }
    if let Some(date) = payload.next_maintenance {
        vehicle_data.next_maintenance = Some(date.into());
    }

    let object = vehicle::UpdateObject {
        id: id.clone(),
        data: Some(vehicle_data),
        mask: Some(FieldMask {
            paths: payload.mask.clone(),
        }),
    };

    grpc_clients
        .storage
        .vehicle
        .update(object)
        .await
        .map_err(|e| {
            rest_error!("(update_aircraft) could not update vehicle: {e}.");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    rest_info!("(update_aircraft) successfully updated aircraft.");
    Ok(())
}

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
) -> Result<(), StatusCode> {
    rest_info!("(remove_aircraft) entry [{}].", &id);

    let id = to_uuid(&id)
        .ok_or_else(|| {
            rest_error!("(remove_aircraft) Invalid aircraft id.");
            StatusCode::BAD_REQUEST
        })? // Check if the aircraft_id is a valid UUID (v4)
        .to_string();

    grpc_clients
        .storage
        .vehicle
        .delete(Id { id })
        .await
        .map_err(|e| {
            rest_error!("(remove_aircraft) could not remove aircraft {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    rest_info!("(remove_aircraft) successfully removed aircraft.");
    Ok(())
}

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
) -> Result<String, StatusCode> {
    rest_info!("(register_aircraft) entry.");
    rest_debug!("(register_aircraft) Payload: {:?}", &payload);

    let id = grpc_clients
        .storage
        .vehicle
        .insert(payload)
        .await
        .map_err(|e| {
            rest_error!("(register_aircraft) could not insert vehicle: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_inner()
        .object
        .ok_or_else(|| {
            rest_error!("(register_aircraft) vehicle insertion failed.");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .id;

    rest_info!("(register_aircraft) registration success.");
    rest_debug!("(register_aircraft) new aircraft: {:?}", id);

    Ok(id)
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
) -> Result<Json<Vec<Aircraft>>, StatusCode> {
    rest_info!("(get_all_aircraft) entry.");

    let filter = AdvancedSearchFilter::search_is_null("deleted_at".to_string());
    let assets = grpc_clients
        .storage
        .vehicle
        .search(filter)
        .await
        .map_err(|e| {
            rest_error!("(get_all_aircraft) could not retrieve vehicles: {e}.");
            StatusCode::NOT_FOUND
        })?
        .into_inner()
        .list
        .into_iter()
        .filter_map(|object| object.try_into().ok())
        .collect();

    Ok(Json(assets))
}

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
) -> Result<Json<Aircraft>, StatusCode> {
    rest_info!("(get_aircraft_by_id) entry [{}].", aircraft_id);
    let id = to_uuid(&aircraft_id)
        .ok_or_else(|| {
            rest_error!("(get_aircraft_by_id) Invalid aircraft id.");
            StatusCode::BAD_REQUEST
        })?
        .to_string(); // Check if the aircraft_id is a valid UUID (v4

    let aircraft: Aircraft = grpc_clients
        .storage
        .vehicle
        .get_by_id(Id { id })
        .await
        .map_err(|e| {
            rest_error!("(get_aircraft_by_id) could not retrieve aircraft: {e}");
            StatusCode::NOT_FOUND
        })?
        .into_inner()
        .try_into()
        .map_err(|e| {
            rest_error!("(get_aircraft_by_id) could not convert vehicle::Object to Aircraft: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(aircraft))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::structs::AssetsInfo;
    use lib_common::logger::get_log_handle;
    use lib_common::uuid::Uuid;

    #[test]
    fn test_aircraft_from_vehicle_object() {
        let expected_vehicle_id = Uuid::new_v4();
        let mut vehicle_data: vehicle::Data = vehicle::mock::get_data_obj();
        let mut vehicle_obj = vehicle::Object {
            id: expected_vehicle_id.clone().to_string(),
            data: Some(vehicle_data.clone()),
        };
        let aircraft_result = vehicle_obj.clone().try_into();
        assert!(aircraft_result.is_ok());
        let aircraft: Aircraft = aircraft_result.unwrap();

        assert_eq!(aircraft.id(), Ok(expected_vehicle_id));
        assert!(aircraft.name().len() > 0);
        assert_eq!(
            aircraft.created_at(),
            vehicle_data
                .created_at
                .clone()
                .expect("mock didn't return a created_at date.")
                .into()
        );
        assert_eq!(
            aircraft.updated_at(),
            vehicle_data
                .updated_at
                .clone()
                .expect("mock didn't return a updated_at date.")
                .into()
        );
        assert_eq!(aircraft.is_grouped(), false);
        assert_eq!(aircraft.is_public(), true);
        assert_eq!(aircraft.status(), AssetStatus::Available);

        vehicle_obj.data = None;
        let result: Result<Aircraft, String> = vehicle_obj.clone().try_into();
        assert_eq!(result.unwrap_err(), "(try_from) vehicle data is missing");

        vehicle_data.updated_at = None;
        vehicle_obj.data = Some(vehicle_data.clone());
        let result: Result<Aircraft, String> = vehicle_obj.clone().try_into();
        assert_eq!(
            result.unwrap_err(),
            "(try_from) updated_at field empty but expected."
        );

        vehicle_data.created_at = None;
        vehicle_data.updated_at = Some(Utc::now().into());
        vehicle_obj.data = Some(vehicle_data.clone());
        let result: Result<Aircraft, String> = vehicle_obj.clone().try_into();
        assert_eq!(
            result.unwrap_err(),
            "(try_from) created_at field empty but expected."
        );
    }

    #[test]
    fn test_aircraft_basics_trait_methods() {
        let aircraft = vehicle::mock::get_data_obj();
        let basics = Basics {
            id: Uuid::new_v4().to_string(),
            name: Some("Test asset".to_string()),
            group_id: Some(Uuid::new_v4().to_string()),
            created_at: aircraft
                .created_at
                .expect("mock didn't return a created_at date.")
                .into(),
            updated_at: aircraft
                .updated_at
                .expect("mock didn't return a updated_at date.")
                .into(),
            whitelist: vec![Uuid::new_v4().to_string()],
            status: AssetStatus::Available,
        };
        let mut aircraft = Aircraft {
            basics: basics.clone(),
            manufacturer: "Airbus".to_string(),
            model: "737-800".to_string(),
            serial_number: aircraft.serial_number,
            registration_number: aircraft.registration_number,
            description: aircraft.description,
            max_payload_kg: 1000.0,
            max_range_km: 1000.0,
            last_maintenance: None,
            next_maintenance: None,
            hangar_id: None,
        };

        assert_eq!(aircraft.id().unwrap(), Uuid::parse_str(&basics.id).unwrap());
        assert_eq!(aircraft.name(), basics.name.clone().unwrap());
        assert_eq!(aircraft.created_at(), basics.created_at);
        assert_eq!(aircraft.updated_at(), basics.updated_at);
        assert_eq!(aircraft.is_grouped(), true);
        assert_eq!(aircraft.is_public(), false);
        assert_eq!(aircraft.status(), basics.status);

        let manufacturer = "Airbus";
        let model = "A320";
        aircraft.basics.name = None;
        aircraft.basics.group_id = None;
        aircraft.manufacturer = manufacturer.to_string();
        aircraft.model = model.to_string();
        assert_eq!(
            aircraft.full_name(),
            format!(
                "{} {} {}",
                manufacturer, model, aircraft.registration_number
            )
        );
        assert_eq!(aircraft.group_id(), Ok(None));
        assert_eq!(aircraft.is_grouped(), false);
    }

    #[tokio::test]
    async fn test_get_all_aircraft() {
        get_log_handle().await;
        ut_info!("(test_get_all_aircraft) Start.");

        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        let payload = vehicle::Data {
            hangar_id: Some(Uuid::new_v4().to_string()),
            hangar_bay_id: Some(Uuid::new_v4().to_string()),
            vehicle_model_id: Uuid::new_v4().to_string(),
            serial_number: Uuid::new_v4().to_string(),
            registration_number: Uuid::new_v4().to_string(),
            description: Some(Uuid::new_v4().to_string()),
            asset_group_id: Some(Uuid::new_v4().to_string()),
            schedule: Some(Uuid::new_v4().to_string()),
            last_maintenance: None,
            next_maintenance: None,
            created_at: None,
            updated_at: None,
        };

        let id = register_aircraft(Extension(grpc_clients.clone()), Json(payload))
            .await
            .unwrap();

        ut_info!("(test_get_all_aircraft) Success: {:#?}", id);

        let result = get_all_aircraft(Extension(grpc_clients)).await.unwrap();
        ut_info!("(test_get_all_aircraft) Success: {:#?}", result);
        // assert!(!result.0.is_empty());
    }

    #[tokio::test]
    async fn test_get_aircraft_by_id() {
        get_log_handle().await;
        ut_info!("(test_get_aircraft_by_id) Start.");

        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        // invalid ID
        let id = "invalid";
        let result =
            get_aircraft_by_id(Extension(grpc_clients.clone()), Path(id.to_string())).await;
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);

        // Valid ID, but doesn't exist
        let id = Uuid::new_v4();
        let error = get_aircraft_by_id(Extension(grpc_clients.clone()), Path(id.to_string()))
            .await
            .unwrap_err();
        assert_eq!(error, StatusCode::NOT_FOUND);

        let data = vehicle::Data {
            hangar_id: Some(Uuid::new_v4().to_string()),
            hangar_bay_id: Some(Uuid::new_v4().to_string()),
            vehicle_model_id: Uuid::new_v4().to_string(),
            serial_number: Uuid::new_v4().to_string(),
            registration_number: Uuid::new_v4().to_string(),
            description: Some(Uuid::new_v4().to_string()),
            asset_group_id: Some(Uuid::new_v4().to_string()),
            schedule: Some(Uuid::new_v4().to_string()),
            last_maintenance: None,
            next_maintenance: None,
            created_at: Some(Utc::now().into()),
            updated_at: Some(Utc::now().into()),
        };

        let id = register_aircraft(Extension(grpc_clients.clone()), Json(data.clone()))
            .await
            .unwrap();

        let expected: Aircraft = vehicle::Object {
            id: id.clone(),
            data: Some(data.clone()),
        }
        .try_into()
        .unwrap();

        let result = get_aircraft_by_id(Extension(grpc_clients.clone()), Path(id.clone()))
            .await
            .unwrap();

        assert_eq!(result.0, expected);
    }

    #[tokio::test]
    async fn test_register_aircraft() {
        get_log_handle().await;
        ut_info!("(test_register_aircraft) Start.");

        let payload = vehicle::Data {
            hangar_id: Some(Uuid::new_v4().to_string()),
            hangar_bay_id: Some(Uuid::new_v4().to_string()),
            vehicle_model_id: Uuid::new_v4().to_string(),
            serial_number: Uuid::new_v4().to_string(),
            registration_number: Uuid::new_v4().to_string(),
            description: Some(Uuid::new_v4().to_string()),
            asset_group_id: Some(Uuid::new_v4().to_string()),
            schedule: Some(Uuid::new_v4().to_string()),
            last_maintenance: None,
            next_maintenance: None,
            created_at: None,
            updated_at: None,
        };

        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);
        register_aircraft(Extension(grpc_clients), Json(payload))
            .await
            .unwrap();

        ut_info!("(test_register_aircraft) Success.");
    }

    #[tokio::test]
    async fn test_update_aircraft() {
        get_log_handle().await;
        ut_info!("(test_update_aircraft) Start.");

        let mut payload = UpdateAircraftPayload {
            id: Uuid::new_v4().to_string(),
            hangar_id: Some(Uuid::new_v4().to_string()),
            hangar_bay_id: Some(Uuid::new_v4().to_string()),
            vehicle_model_id: Some(Uuid::new_v4().to_string()),
            serial_number: Some(Uuid::new_v4().to_string()),
            registration_number: Some(Uuid::new_v4().to_string()),
            description: Some(Uuid::new_v4().to_string()),
            asset_group_id: Some(Uuid::new_v4().to_string()),
            schedule: Some(Uuid::new_v4().to_string()),
            last_maintenance: None,
            next_maintenance: None,
            mask: vec![],
        };

        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        // invalid ID
        payload.id = "invalid".to_string();
        let error = update_aircraft(Extension(grpc_clients.clone()), Json(payload.clone()))
            .await
            .unwrap_err();
        assert_eq!(error, StatusCode::BAD_REQUEST);

        // Valid ID, but doesn't exist
        payload.id = Uuid::new_v4().to_string();
        let error = update_aircraft(Extension(grpc_clients.clone()), Json(payload.clone()))
            .await
            .unwrap_err();
        assert_eq!(error, StatusCode::NOT_FOUND);
        ut_info!("(test_update_aircraft) Success.");
    }

    #[tokio::test]
    async fn test_remove_aircraft() {
        get_log_handle().await;
        ut_info!("(test_remove_aircraft) Start.");

        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        // Invalid ID
        let id = "invalid";
        let result = remove_aircraft(Extension(grpc_clients.clone()), Path(id.to_string())).await;
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);

        // Valid ID
        let id = Uuid::new_v4();
        remove_aircraft(Extension(grpc_clients.clone()), Path(id.to_string()))
            .await
            .unwrap();
        ut_info!("(test_remove_aircraft) Success.");
    }
}
