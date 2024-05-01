//! Handlers for the vertipad API endpoints.

pub use super::rest_types::UpdateVertipadPayload;

use crate::rest::structs::{AssetStatus, Basics};
use axum::{extract::Path, Extension, Json};
use hyper::StatusCode;
use svc_storage_client_grpc::prelude::*;
use svc_storage_client_grpc::resources::vertipad;

use crate::grpc::client::GrpcClients;
use lib_common::uuid::to_uuid;

/// The default name for a vertipad
const VERTIPAD_NAME: &str = "Unnamed Vertipad";

/// A struct representing a vertipad (a vertical landing pad).
///
/// A vertipad is a landing pad that is used for vertical takeoff and
/// landing (VTOL) aircraft. Usually, vertipads belong to vertiports.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Vertipad {
    /// Attributes that are common to all assets.
    pub basics: Basics,

    /// The vertiport ID that the vertipad belongs to.
    pub vertiport_id: String,

    /// Whether the vertipad is enabled.
    pub enabled: bool,

    /// Whether the vertipad is occupied.
    pub occupied: bool,

    /// The geographical location of the vertipad.
    pub geo_location: GeoPoint,

    /// The schedule of the vertipad.
    pub schedule: Option<String>,
}

impl Vertipad {
    /// Get the vertipad's name.
    ///
    /// It is recommended to make the `name` field required for
    /// vertiports. This can be done through frontend validation.
    pub fn full_name(&self) -> String {
        match &self.basics.name {
            Some(name) => name.clone(),
            None => VERTIPAD_NAME.to_string(),
        }
    }
}

impl TryFrom<vertipad::Object> for Vertipad {
    type Error = String;

    fn try_from(object: vertipad::Object) -> Result<Self, Self::Error> {
        let data = match object.data {
            Some(data) => data,
            None => return Err("(try_from) vertipad data is missing".to_string()),
        };

        Ok(Vertipad {
            basics: Basics {
                id: object.id,
                group_id: None,
                name: Some(data.name),
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
            geo_location: data
                .geo_location
                .ok_or("(try_from) geo_location empty but expected.")?,
            vertiport_id: data.vertiport_id,
            enabled: data.enabled,
            occupied: data.occupied,
            schedule: data.schedule,
        })
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
) -> Result<String, StatusCode> {
    rest_info!("(register_vertipad) entry.");
    rest_debug!("(register_vertipad) Payload: {:?}", &payload);

    // TODO(R5): maybe not safe to just take the storage type directly from the client and shove
    //  it into svc-storage without parsing/checking
    let id = grpc_clients
        .storage
        .vertipad
        .insert(payload)
        .await
        .map_err(|e| {
            rest_error!("(register_vertipad) could not insert vertipad: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_inner()
        .object
        .ok_or_else(|| {
            rest_error!("(register_vertipad) could not insert vertipad.");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .id;

    rest_info!("(register_vertipad) registration success.");

    Ok(id)
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
) -> Result<(), StatusCode> {
    rest_info!("(update_vertipad) entry [{}].", payload.id);
    rest_debug!("(update_vertipad) Payload: {:?}", &payload);

    let id = to_uuid(&payload.id)
        .ok_or_else(|| {
            let error_msg = "Invalid vertipad id".to_string();
            rest_error!("(update_vertipad) {}", &error_msg);
            StatusCode::BAD_REQUEST
        })?
        .to_string();

    let mut vertipad_data = grpc_clients
        .storage
        .vertipad
        .get_by_id(Id { id: id.clone() })
        .await
        .map_err(|e| {
            let error_msg = format!("could not retrieve vertipad: {}", e);
            rest_error!("(update_vertipad) {}", &error_msg);
            StatusCode::NOT_FOUND
        })?
        .into_inner()
        .data
        .ok_or_else(|| {
            let error_msg = "vertipad not found".to_string();
            rest_error!("(update_vertipad) {}", &error_msg);
            StatusCode::NOT_FOUND
        })?;

    if let Some(name) = payload.name {
        vertipad_data.name = name;
    }

    if let Some(vertiport_id) = payload.vertiport_id {
        vertipad_data.vertiport_id = vertiport_id;
    }

    if let Some(enabled) = payload.enabled {
        vertipad_data.enabled = enabled;
    }

    if let Some(occupied) = payload.occupied {
        vertipad_data.occupied = occupied;
    }

    vertipad_data.geo_location = payload.geo_location;

    let object = vertipad::UpdateObject {
        id: id.clone(),
        data: Some(vertipad_data),
        mask: Some(FieldMask {
            paths: payload.mask,
        }),
    };

    grpc_clients
        .storage
        .vertipad
        .update(object)
        .await
        .map_err(|e| {
            let error_msg = format!("could not update vertipad: {}", e);
            rest_error!("(update_vertipad) {}", &error_msg);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    rest_info!("(update_vertipad) successfully updated vertipad.");
    Ok(())
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
) -> Result<(), StatusCode> {
    rest_info!("(remove_vertipad) entry [{}].", &id);

    let id = to_uuid(&id)
        .ok_or_else(|| {
            rest_error!("(remove_vertipad) Invalid vertipad id: {}", &id);
            StatusCode::BAD_REQUEST
        })?
        .to_string();

    grpc_clients
        .storage
        .vertipad
        .delete(Id { id })
        .await
        .map_err(|e| {
            rest_error!("(remove_vertipad) could not remove vertipad: {e}");
            StatusCode::NOT_FOUND
        })?;

    Ok(())
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
) -> Result<Json<Vec<Vertipad>>, StatusCode> {
    rest_info!("(get_all_vertipads) entry.");
    let filter = AdvancedSearchFilter::search_is_null("deleted_at".to_string());
    let assets: Vec<Vertipad> = grpc_clients
        .storage
        .vertipad
        .search(filter)
        .await
        .map_err(|e| {
            rest_error!("(get_all_vertipads) could not retrieve vertipads: {e}.");
            StatusCode::NOT_FOUND
        })?
        .into_inner()
        .list
        .into_iter()
        .filter_map(|object| object.try_into().ok())
        .collect();

    Ok(Json(assets))
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
) -> Result<Json<Vertipad>, StatusCode> {
    rest_info!("(get_vertipad_by_id) entry [{}].", vertipad_id);

    let id = to_uuid(&vertipad_id)
        .ok_or_else(|| {
            rest_error!("(get_vertipad_by_id) Invalid vertipad id: {}", vertipad_id);
            StatusCode::BAD_REQUEST
        })? // Check if the vertipad_id is a valid UUID
        .to_string();

    let vertipad: Vertipad = grpc_clients
        .storage
        .vertipad
        .get_by_id(Id { id })
        .await
        .map_err(|e| {
            rest_error!("(get_vertipad_by_id) could not get vertipad: {e}");
            StatusCode::NOT_FOUND
        })?
        .into_inner()
        .try_into()
        .map_err(|e| {
            rest_error!("(get_vertipad_by_id) could not convert vertipad: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    rest_debug!("(get_vertipad_by_id) vertipad found: {:#?}", vertipad);
    Ok(Json(vertipad))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grpc::client::GrpcClients;
    use crate::rest::structs::AssetsInfo;
    use axum::extract::Extension;
    use lib_common::time::Utc;
    use lib_common::uuid::Uuid;
    use svc_storage_client_grpc::resources::vertipad;

    #[test]
    fn test_vertipad_basics_trait_methods() {
        let vertipad = vertipad::mock::get_data_obj();
        let expected_name = "Test asset".to_string();
        let basics = Basics {
            id: Uuid::new_v4().to_string(),
            name: Some(expected_name.clone()),
            group_id: Some(Uuid::new_v4().to_string()),
            created_at: vertipad
                .created_at
                .expect("mock didn't return a created_at date.")
                .into(),
            updated_at: vertipad
                .updated_at
                .expect("mock didn't return a updated_at date.")
                .into(),
            whitelist: vec![Uuid::new_v4().to_string()],
            status: AssetStatus::Available,
        };

        let mut vertipad = Vertipad {
            basics: basics.clone(),
            geo_location: vertipad.geo_location.unwrap(),
            vertiport_id: vertipad.vertiport_id,
            enabled: vertipad.enabled,
            occupied: vertipad.occupied,
            schedule: vertipad.schedule,
        };
        assert_eq!(vertipad.id().unwrap(), Uuid::parse_str(&basics.id).unwrap());
        assert_eq!(vertipad.name(), expected_name);
        assert_eq!(vertipad.created_at(), basics.created_at);
        assert_eq!(vertipad.updated_at(), basics.updated_at);
        assert_eq!(vertipad.is_grouped(), true);
        assert_eq!(vertipad.is_public(), false);
        assert_eq!(vertipad.status(), basics.status);

        vertipad.basics.name = None;
        assert_eq!(vertipad.full_name(), VERTIPAD_NAME);

        vertipad.basics.group_id = None;
        assert_eq!(vertipad.is_grouped(), false);
        assert_eq!(vertipad.group_id(), Ok(None));
    }

    #[test]
    fn test_vertipad_from_vertipad_object() {
        let expected_vertipad_id = Uuid::new_v4();
        let mut vertipad_data: vertipad::Data = vertipad::mock::get_data_obj();
        let mut vertipad_obj = vertipad::Object {
            id: expected_vertipad_id.clone().to_string(),
            data: Some(vertipad_data.clone()),
        };
        let vertipad_result = vertipad_obj.clone().try_into();
        assert!(vertipad_result.is_ok());
        let vertipad: Vertipad = vertipad_result.unwrap();

        assert_eq!(vertipad.id(), Ok(expected_vertipad_id));
        assert_eq!(vertipad.name(), vertipad_data.name);
        assert_eq!(
            vertipad.created_at(),
            vertipad_data
                .created_at
                .clone()
                .expect("mock didn't return a created_at date.")
                .into()
        );
        assert_eq!(
            vertipad.updated_at(),
            vertipad_data
                .updated_at
                .clone()
                .expect("mock didn't return a updated_at date.")
                .into()
        );
        assert_eq!(vertipad.is_grouped(), false);
        assert_eq!(vertipad.is_public(), true);
        assert_eq!(vertipad.status(), AssetStatus::Available);

        vertipad_obj.data = None;
        let result: Result<Vertipad, String> = vertipad_obj.clone().try_into();
        assert_eq!(result.unwrap_err(), "(try_from) vertipad data is missing");

        vertipad_data.updated_at = None;
        vertipad_obj.data = Some(vertipad_data.clone());
        let result: Result<Vertipad, String> = vertipad_obj.clone().try_into();
        assert_eq!(
            result.unwrap_err(),
            "(try_from) updated_at field empty but expected."
        );

        vertipad_data.created_at = None;
        vertipad_data.updated_at = Some(Utc::now().into());
        vertipad_obj.data = Some(vertipad_data.clone());
        let result: Result<Vertipad, String> = vertipad_obj.clone().try_into();
        assert_eq!(
            result.unwrap_err(),
            "(try_from) created_at field empty but expected."
        );

        vertipad_data.created_at = Some(Utc::now().into());
        vertipad_data.geo_location = None;
        vertipad_obj.data = Some(vertipad_data.clone());
        let result: Result<Vertipad, String> = vertipad_obj.clone().try_into();
        assert_eq!(
            result.unwrap_err(),
            "(try_from) geo_location empty but expected."
        );
    }

    #[tokio::test]
    async fn test_register_vertipad() {
        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        let vertipad_data = vertipad::Data {
            name: "Test Vertipad".to_string(),
            vertiport_id: "test-vertiport".to_string(),
            geo_location: Some(GeoPoint {
                latitude: 0.0,
                longitude: 0.0,
                altitude: 0.0,
            }),
            enabled: true,
            occupied: false,
            schedule: None,
            created_at: None,
            updated_at: None,
        };

        let response = register_vertipad(Extension(grpc_clients), Json(vertipad_data))
            .await
            .expect("Failed to register vertipad");

        // Should be valid UUID
        to_uuid(&response).unwrap();
    }

    #[tokio::test]
    async fn test_get_all_vertipads() {
        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        let data = vertipad::Data {
            name: "Test Vertipad".to_string(),
            vertiport_id: "test-vertiport".to_string(),
            geo_location: Some(GeoPoint {
                latitude: 0.0,
                longitude: 0.0,
                altitude: 0.0,
            }),
            enabled: true,
            occupied: false,
            schedule: None,
            created_at: Some(Utc::now().into()),
            updated_at: Some(Utc::now().into()),
        };

        let _ = register_vertipad(Extension(grpc_clients.clone()), Json(data))
            .await
            .unwrap();

        let response = get_all_vertipads(Extension(grpc_clients)).await.unwrap();
        assert!(!response.0.is_empty());
    }

    #[tokio::test]
    async fn test_remove_vertipad() {
        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        let vertipad_data = vertipad::Data {
            name: "Test Vertipad".to_string(),
            vertiport_id: "test-vertiport".to_string(),
            geo_location: Some(GeoPoint {
                latitude: 0.0,
                longitude: 0.0,
                altitude: 0.0,
            }),
            enabled: true,
            occupied: false,
            schedule: None,
            created_at: None,
            updated_at: None,
        };

        // INVALID UUID
        let error = remove_vertipad(Extension(grpc_clients.clone()), Path("invalid".to_string()))
            .await
            .unwrap_err();
        assert_eq!(error, StatusCode::BAD_REQUEST);

        // // NONEXISTENT UUID
        // let error = remove_vertipad(Extension(grpc_clients.clone()), Path(Uuid::new_v4().to_string()))
        //     .await
        //     .unwrap_err();
        // assert_eq!(error, StatusCode::NOT_FOUND);

        // EXISTING UUID
        let vertipad_id = register_vertipad(Extension(grpc_clients.clone()), Json(vertipad_data))
            .await
            .expect("Failed to register vertipad");

        let response = remove_vertipad(Extension(grpc_clients), Path(vertipad_id))
            .await
            .expect("Failed to remove vertipad");

        assert_eq!(response, ());
    }

    #[tokio::test]
    async fn test_get_vertipad_by_id() {
        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        // INVALID UUID
        let error =
            get_vertipad_by_id(Extension(grpc_clients.clone()), Path("invalid".to_string()))
                .await
                .unwrap_err();
        assert_eq!(error, StatusCode::BAD_REQUEST);

        // NONEXISTENT UUID
        let error = get_vertipad_by_id(
            Extension(grpc_clients.clone()),
            Path(Uuid::new_v4().to_string()),
        )
        .await
        .unwrap_err();
        assert_eq!(error, StatusCode::NOT_FOUND);

        let data = vertipad::Data {
            name: "Test Vertipad".to_string(),
            vertiport_id: "test-vertiport".to_string(),
            geo_location: Some(GeoPoint {
                latitude: 0.0,
                longitude: 0.0,
                altitude: 0.0,
            }),
            enabled: true,
            occupied: false,
            schedule: None,
            created_at: Some(Utc::now().into()),
            updated_at: Some(Utc::now().into()),
        };

        let id = register_vertipad(Extension(grpc_clients.clone()), Json(data.clone()))
            .await
            .expect("Failed to register vertipad");

        let response = get_vertipad_by_id(Extension(grpc_clients), Path(id.clone()))
            .await
            .unwrap();

        let expected: Vertipad = vertipad::Object {
            id: id.clone(),
            data: Some(data),
        }
        .try_into()
        .unwrap();

        assert_eq!(response.0, expected);
    }

    #[tokio::test]
    async fn test_update_vertipad() {
        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        let mut payload = UpdateVertipadPayload {
            id: "invalid".to_string(),
            vertiport_id: Some(Uuid::new_v4().to_string()),
            mask: vec!["name".to_string()],
            name: Some("Updated Vertipad".to_string()),
            geo_location: None,
            enabled: None,
            occupied: None,
            schedule: None,
        };

        // INVALID UUID in request
        let error = update_vertipad(Extension(grpc_clients.clone()), Json(payload.clone()))
            .await
            .unwrap_err();
        assert_eq!(error, StatusCode::BAD_REQUEST);

        // NONEXISTENT UUID
        payload.id = Uuid::new_v4().to_string();
        let error = update_vertipad(Extension(grpc_clients.clone()), Json(payload.clone()))
            .await
            .unwrap_err();
        assert_eq!(error, StatusCode::NOT_FOUND);

        // VALID
        let vertipad_data = vertipad::Data {
            name: "Test Vertipad".to_string(),
            vertiport_id: "test-vertiport".to_string(),
            geo_location: Some(GeoPoint {
                latitude: 0.0,
                longitude: 0.0,
                altitude: 0.0,
            }),
            enabled: true,
            occupied: false,
            schedule: None,
            created_at: None,
            updated_at: None,
        };

        payload.id = register_vertipad(Extension(grpc_clients.clone()), Json(vertipad_data))
            .await
            .expect("Failed to register vertipad");
        let response = update_vertipad(Extension(grpc_clients), Json(payload.clone()))
            .await
            .expect("Failed to update vertipad");

        assert_eq!(response, ());
    }
}
