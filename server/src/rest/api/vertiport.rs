//! Handlers for the vertiport API endpoints.

pub use super::rest_types::UpdateVertiportPayload;

use crate::rest::structs::{AssetStatus, Basics};
use axum::{extract::Path, Extension, Json};
use hyper::StatusCode;
use svc_storage_client_grpc::prelude::*;
use svc_storage_client_grpc::resources::vertiport;

use crate::grpc::client::GrpcClients;
use lib_common::uuid::to_uuid;

/// The default name for a vertiport
const VERTIPORT_NAME: &str = "Unnamed Vertiport";

/// A struct representing a vertiport (a vertical airport).
///
/// A vertiport is an airport that is used for vertical takeoff and
/// landing (VTOL) aircraft. A vertiport may have one or more vertipads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Vertiport {
    /// Attributes that are common to all assets.
    pub basics: Basics,

    /// A description of the vertiport.
    pub description: String,

    /// The geographic location of the vertiport.
    pub geo_location: GeoPolygon,

    /// The schedule of the vertiport.
    pub schedule: Option<String>,
}

impl Vertiport {
    /// Get the vertiport's name.
    ///
    /// It is recommended to make the `name` field required for
    /// vertiports. This can be done through frontend validation.
    pub fn full_name(&self) -> String {
        match &self.basics.name {
            Some(name) => name.clone(),
            None => VERTIPORT_NAME.to_string(),
        }
    }
}

impl TryFrom<vertiport::Object> for Vertiport {
    type Error = String;

    fn try_from(object: vertiport::Object) -> Result<Self, Self::Error> {
        let data = match object.data {
            Some(data) => data,
            None => return Err("(try_from) vertiport data is missing".to_string()),
        };

        Ok(Vertiport {
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
            description: data.description,
            geo_location: data
                .geo_location
                .ok_or("(try_from) geo_location empty but expected.")?,
            schedule: data.schedule,
        })
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
) -> Result<String, StatusCode> {
    rest_info!("(register_vertiport) entry.");
    rest_debug!("(register_vertiport) Payload: {:?}", &payload);

    let id = grpc_clients
        .storage
        .vertiport
        .insert(payload)
        .await
        .map_err(|e| {
            rest_error!("(register_vertiport) could not insert vertiport: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_inner()
        .object
        .ok_or_else(|| {
            rest_error!("(register_vertiport) vertiport not found.");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .id;

    Ok(id)
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
) -> Result<(), StatusCode> {
    rest_info!("(update_vertiport) entry [{}].", payload.id);
    rest_debug!("(update_vertiport) Payload: {:?}", &payload);

    let id = to_uuid(&payload.id)
        .ok_or_else(|| {
            rest_error!("(update_vertiport) Invalid vertiport id: {}", &payload.id);
            StatusCode::BAD_REQUEST
        })?
        .to_string();

    let mut vertiport_data = grpc_clients
        .storage
        .vertiport
        .get_by_id(Id { id: id.clone() })
        .await
        .map_err(|e| {
            rest_error!("(update_vertiport) error getting vertiport from storage: {e}");
            StatusCode::NOT_FOUND
        })?
        .into_inner()
        .data
        .ok_or_else(|| {
            rest_error!("(update_vertiport) vertiport data malformed.");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    vertiport_data.geo_location = payload.geo_location;
    vertiport_data.schedule = payload.schedule;
    if let Some(name) = payload.name {
        vertiport_data.name = name;
    }

    if let Some(description) = payload.description {
        vertiport_data.description = description;
    }

    let object = vertiport::UpdateObject {
        id,
        data: Some(vertiport_data),
        mask: Some(FieldMask {
            paths: payload.mask,
        }),
    };

    grpc_clients
        .storage
        .vertiport
        .update(object)
        .await
        .map_err(|e| {
            rest_error!("(update_vertiport) could not update vertiport: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    rest_info!("(update_vertiport) successfully updated vertiport.",);

    Ok(())
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
) -> Result<(), StatusCode> {
    rest_info!("(remove_vertiport) entry [{}].", &id);

    let id = to_uuid(&id)
        .ok_or_else(|| {
            rest_error!("(remove_vertiport) Invalid vertiport id: {}", &id);
            StatusCode::BAD_REQUEST
        })?
        .to_string();

    grpc_clients
        .storage
        .vertiport
        .delete(Id { id })
        .await
        .map_err(|e| {
            rest_error!("(remove_vertiport) could not remove vertiport: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(())
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
) -> Result<Json<Vec<Vertiport>>, StatusCode> {
    rest_info!("(get_all_vertiports) entry.");
    let filter = AdvancedSearchFilter::search_is_null("deleted_at".to_string());
    let assets: Vec<Vertiport> = grpc_clients
        .storage
        .vertiport
        .search(filter)
        .await
        .map_err(|e| {
            rest_error!("(get_all_vertiports) could not retrieve vertiports: {e}.");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_inner()
        .list
        .into_iter()
        .filter_map(|object| object.try_into().ok())
        .collect();

    Ok(Json(assets))
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
) -> Result<Json<Vertiport>, StatusCode> {
    rest_info!("(get_vertiport_by_id) entry [{}].", vertiport_id);

    let id = to_uuid(&vertiport_id)
        .ok_or_else(|| {
            rest_error!(
                "(get_vertiport_by_id) Invalid vertiport id: {}",
                vertiport_id
            );
            StatusCode::BAD_REQUEST
        })?
        .to_string();

    let vertiport: Vertiport = grpc_clients
        .storage
        .vertiport
        .get_by_id(Id { id })
        .await
        .map_err(|e| {
            rest_error!("(get_vertiport_by_id) error getting vertiport from storage: {e}");
            StatusCode::NOT_FOUND
        })?
        .into_inner()
        .try_into()
        .map_err(|e| {
            rest_error!("(get_vertiport_by_id) error converting vehicle to vertiport: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    rest_info!("(get_vertiport_by_id) Vertiport found: {}", vertiport_id);

    Ok(Json(vertiport))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::structs::AssetsInfo;
    use lib_common::time::Utc;
    use lib_common::uuid::Uuid;

    #[test]
    fn test_vertiport_basics_trait_methods() {
        let vertiport = vertiport::mock::get_data_obj();
        let basics = Basics {
            id: Uuid::new_v4().to_string(),
            name: Some("Test asset".to_string()),
            group_id: Some(Uuid::new_v4().to_string()),
            created_at: vertiport
                .created_at
                .expect("mock didn't return a created_at date.")
                .into(),
            updated_at: vertiport
                .updated_at
                .expect("mock didn't return a updated_at date.")
                .into(),
            whitelist: vec![Uuid::new_v4().to_string()],
            status: AssetStatus::Available,
        };

        let mut vertiport = Vertiport {
            basics: basics.clone(),
            description: vertiport.description,
            geo_location: vertiport.geo_location.unwrap(),
            schedule: vertiport.schedule,
        };
        assert_eq!(
            vertiport.id().unwrap(),
            Uuid::parse_str(&basics.id).unwrap()
        );
        assert_eq!(vertiport.name(), basics.name.clone().unwrap());
        assert_eq!(vertiport.created_at(), basics.created_at);
        assert_eq!(vertiport.updated_at(), basics.updated_at);
        assert_eq!(vertiport.is_grouped(), true);
        assert_eq!(vertiport.is_public(), false);
        assert_eq!(vertiport.status(), basics.status);

        vertiport.basics.name = None;
        assert_eq!(vertiport.full_name(), VERTIPORT_NAME);

        vertiport.basics.group_id = None;
        assert_eq!(vertiport.is_grouped(), false);
        assert_eq!(vertiport.group_id(), Ok(None));
    }

    #[test]
    fn test_vertiport_from_vertiport_object() {
        let expected_vertiport_id = Uuid::new_v4();
        let mut vertiport_data: vertiport::Data = vertiport::mock::get_data_obj();
        let mut vertiport_obj = vertiport::Object {
            id: expected_vertiport_id.clone().to_string(),
            data: Some(vertiport_data.clone()),
        };
        let vertiport_result = vertiport_obj.clone().try_into();
        assert!(vertiport_result.is_ok());
        let vertiport: Vertiport = vertiport_result.unwrap();

        assert_eq!(vertiport.id(), Ok(expected_vertiport_id));
        assert_eq!(vertiport.name(), vertiport_data.name);
        assert_eq!(
            vertiport.created_at(),
            vertiport_data
                .created_at
                .clone()
                .expect("mock didn't return a created_at date.")
                .into()
        );
        assert_eq!(
            vertiport.updated_at(),
            vertiport_data
                .updated_at
                .clone()
                .expect("mock didn't return a updated_at date.")
                .into()
        );
        assert_eq!(vertiport.is_grouped(), false);
        assert_eq!(vertiport.is_public(), true);
        assert_eq!(vertiport.status(), AssetStatus::Available);

        vertiport_obj.data = None;
        let result: Result<Vertiport, String> = vertiport_obj.clone().try_into();
        assert_eq!(result.unwrap_err(), "(try_from) vertiport data is missing");

        vertiport_data.updated_at = None;
        vertiport_obj.data = Some(vertiport_data.clone());
        let result: Result<Vertiport, String> = vertiport_obj.clone().try_into();
        assert_eq!(
            result.unwrap_err(),
            "(try_from) updated_at field empty but expected."
        );

        vertiport_data.created_at = None;
        vertiport_data.updated_at = Some(Utc::now().into());
        vertiport_obj.data = Some(vertiport_data.clone());
        let result: Result<Vertiport, String> = vertiport_obj.clone().try_into();
        assert_eq!(
            result.unwrap_err(),
            "(try_from) created_at field empty but expected."
        );

        vertiport_data.created_at = Some(Utc::now().into());
        vertiport_data.geo_location = None;
        vertiport_obj.data = Some(vertiport_data.clone());
        let result: Result<Vertiport, String> = vertiport_obj.clone().try_into();
        assert_eq!(
            result.unwrap_err(),
            "(try_from) geo_location empty but expected."
        );
    }

    #[tokio::test]
    async fn test_register_vertiport() {
        let vertiport_data = vertiport::mock::get_data_obj();
        let vertiport_data = vertiport::Data {
            name: vertiport_data.name,
            description: vertiport_data.description,
            geo_location: vertiport_data.geo_location,
            schedule: vertiport_data.schedule,
            created_at: None,
            updated_at: None,
        };

        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);
        let result = register_vertiport(Extension(grpc_clients), Json(vertiport_data.clone()))
            .await
            .unwrap();

        // Confirm valid UUID
        to_uuid(&result).unwrap();
    }

    #[tokio::test]
    async fn test_update_vertiport() {
        let data = vertiport::mock::get_data_obj();
        let data = vertiport::Data {
            geo_location: Some(GeoPolygon {
                interiors: vec![],
                exterior: None,
            }),
            created_at: Some(Utc::now().into()),
            updated_at: Some(Utc::now().into()),
            ..data
        };

        let mut payload = UpdateVertiportPayload {
            id: Uuid::new_v4().to_string(),
            name: Some("Test Vertiport".to_string()),
            description: Some("Test description".to_string()),
            geo_location: None,
            schedule: Some("Test schedule".to_string()),
            mask: vec!["name".to_string()],
        };

        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        // INVALID UUID
        payload.id = "invalid".to_string();
        let result = update_vertiport(Extension(grpc_clients.clone()), Json(payload.clone()))
            .await
            .unwrap_err();
        assert_eq!(result, StatusCode::BAD_REQUEST);

        // NONEXISTENT RECORD
        payload.id = Uuid::new_v4().to_string();
        let result = update_vertiport(Extension(grpc_clients.clone()), Json(payload.clone()))
            .await
            .unwrap_err();

        assert_eq!(result, StatusCode::NOT_FOUND);

        payload.id = grpc_clients
            .storage
            .vertiport
            .insert(data.clone())
            .await
            .unwrap()
            .into_inner()
            .object
            .unwrap()
            .id;

        // Update record
        update_vertiport(Extension(grpc_clients.clone()), Json(payload.clone()))
            .await
            .unwrap();

        // let expected = vertiport::Object {
        //     id: payload.id.clone(),
        //     data
        // }
        // .try_into()
        // .unwrap();
    }

    #[tokio::test]
    async fn test_remove_vertiport() {
        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        // INVALID UUID
        let result = remove_vertiport(Extension(grpc_clients.clone()), Path("invalid".to_string()))
            .await
            .unwrap_err();
        assert_eq!(result, StatusCode::BAD_REQUEST);

        // NONEXISTENT RECORD will return Ok(())
        remove_vertiport(
            Extension(grpc_clients.clone()),
            Path(Uuid::new_v4().to_string()),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_get_all_vertiports() {
        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        let data = vertiport::Data {
            name: "Test".to_string(),
            description: "Description".to_string(),
            geo_location: Some(GeoPolygon {
                interiors: vec![],
                exterior: None,
            }),
            schedule: None,
            created_at: Some(Utc::now().into()),
            updated_at: Some(Utc::now().into()),
        };

        let _ = register_vertiport(Extension(grpc_clients.clone()), Json(data.clone()))
            .await
            .expect("Failed to register vertipad");

        let response = get_all_vertiports(Extension(grpc_clients.clone()))
            .await
            .unwrap();

        assert!(!response.0.is_empty());
    }

    #[tokio::test]
    async fn test_get_vertiport_by_id() {
        let data = vertiport::Data {
            name: "Test".to_string(),
            description: "Description".to_string(),
            geo_location: Some(GeoPolygon {
                interiors: vec![],
                exterior: None,
            }),
            schedule: None,
            created_at: Some(Utc::now().into()),
            updated_at: Some(Utc::now().into()),
        };

        let config = crate::config::Config::default();
        let grpc_clients = GrpcClients::default(config);

        // INVALID UUID
        let result =
            get_vertiport_by_id(Extension(grpc_clients.clone()), Path("invalid".to_string()))
                .await
                .unwrap_err();
        assert_eq!(result, StatusCode::BAD_REQUEST);

        // NONEXISTENT RECORD
        let result = get_vertiport_by_id(
            Extension(grpc_clients.clone()),
            Path(Uuid::new_v4().to_string()),
        )
        .await
        .unwrap_err();
        assert_eq!(result, StatusCode::NOT_FOUND);

        // Insert new record
        let id = register_vertiport(Extension(grpc_clients.clone()), Json(data.clone()))
            .await
            .expect("Failed to register vertipad");

        // Get record
        let result = get_vertiport_by_id(Extension(grpc_clients.clone()), Path(id.clone()))
            .await
            .unwrap();

        let expected: Vertiport = vertiport::Object {
            id: id.clone(),
            data: Some(data),
        }
        .try_into()
        .unwrap();

        assert_eq!(result.0, expected);
    }
}
