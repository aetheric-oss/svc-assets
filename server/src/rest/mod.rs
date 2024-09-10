//! REST
//! provides server implementations for REST API

#[macro_use]
pub mod macros;
pub mod api;
pub mod server;
pub mod structs;

pub use api::rest_types::*;

use std::fmt::{self, Display, Formatter};
use utoipa::OpenApi;

/// OpenAPI 3.0 specification for this service
#[derive(OpenApi, Copy, Clone, Debug)]
#[openapi(
    paths(
        api::health::health_check,

        // GET
        api::operator::get_operator,
        api::operator::get_all_assets_by_operator,
        api::operator::get_all_grouped_assets,
        api::operator::get_all_grouped_assets_delegated_to,
        api::operator::get_all_grouped_assets_delegated_from,
        api::aircraft::get_all_aircraft,
        api::vertiport::get_all_vertiports,
        api::vertipad::get_all_vertipads,
        api::aircraft::get_aircraft_by_id,
        api::vertipad::get_vertipad_by_id,
        api::vertiport::get_vertiport_by_id,
        api::group::get_asset_group_by_id,

        // CREATE
        api::aircraft::register_aircraft,
        api::vertiport::register_vertiport,
        api::vertipad::register_vertipad,
        api::group::register_asset_group,

        // UPDATE
        api::aircraft::update_aircraft,
        api::vertiport::update_vertiport,
        api::vertipad::update_vertipad,
        api::group::update_asset_group,

        // REMOVE
        api::aircraft::remove_aircraft,
        api::vertiport::remove_vertiport,
        api::vertipad::remove_vertipad,
        api::group::remove_asset_group,
    ),
    components(
        schemas(
            vehicle::Data,
            vertiport::Data,
            vertipad::Data,
            GeoPointZ,
            GeoPolygonZ,
            GeoLineStringZ,
            RegisterAssetGroupPayload,
            UpdateAircraftPayload,
            UpdateVertiportPayload,
            UpdateVertipadPayload,
            api::operator::Operator,
            api::aircraft::Aircraft,
            api::vertiport::Vertiport,
            api::vertipad::Vertipad,
            structs::AssetGroup,
            structs::AssetStatus,
            structs::Basics,
        )
    ),
    tags(
        (name = "svc-assets", description = "svc-assets API")
    )
)]
#[cfg(not(tarpaulin_include))]
// no_coverage: (Rnever) not unit testable
pub struct ApiDoc;

/// Errors with OpenAPI generation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpenApiError {
    /// Failed to export as JSON string
    Json,

    /// Failed to write to file
    FileWrite,
}

impl std::error::Error for OpenApiError {}

impl Display for OpenApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OpenApiError::Json => write!(f, "Failed to export as JSON string"),
            OpenApiError::FileWrite => write!(f, "Failed to write to file"),
        }
    }
}

/// Create OpenAPI 3.0 Specification File
pub fn generate_openapi_spec<T>(target: &str) -> Result<(), OpenApiError>
where
    T: OpenApi,
{
    let output = T::openapi().to_pretty_json().map_err(|e| {
        rest_error!("failed to export as JSON string: {e}");
        OpenApiError::Json
    })?;

    std::fs::write(target, output).map_err(|e| {
        rest_error!("failed to write to file: {e}");
        OpenApiError::FileWrite
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_openapi_spec() {
        let target = "/nonsense/";
        let error = generate_openapi_spec::<ApiDoc>(target).unwrap_err();
        assert_eq!(error, OpenApiError::FileWrite);

        // TODO(R5): Is it possible to make the JSON export fail?
        // #[derive(OpenApi)]
        // #[openapi(
        //     paths(invalid)
        // )]
        // struct InvalidApi;
        // let error = generate_openapi_spec::<InvalidApi>("test.json").unwrap_err();
        // assert_eq!(error, OpenApiError::Json);
    }
}
