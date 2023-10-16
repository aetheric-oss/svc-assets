//! REST
//! provides server implementations for REST API

#[macro_use]
pub mod macros;
pub mod api;
pub mod server;

use utoipa::OpenApi;

use api::rest_types::*;
use api::structs;
use svc_storage_client_grpc::prelude::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        //
        // asset endpoints
        //

        // get
        api::asset::get_all_aircraft,
        api::asset::get_all_vertiports,
        api::asset::get_all_vertipads,
        api::asset::get_aircraft_by_id,
        api::asset::get_vertipad_by_id,
        api::asset::get_vertiport_by_id,

        // new
        api::asset::register_aircraft,
        api::asset::register_vertiport,
        api::asset::register_vertipad,

        // update
        api::asset::update_aircraft,
        api::asset::update_vertiport,
        api::asset::update_vertipad,

        // remove
        api::asset::remove_aircraft,
        api::asset::remove_vertiport,
        api::asset::remove_vertipad,

        //
        // operator endpoints
        //
        api::operator::get_all_assets_by_operator,
        api::operator::get_operator,

        //
        // group endpoints
        //
        api::group::get_all_grouped_assets,
        api::group::get_all_grouped_assets_delegated_to,
        api::group::get_all_grouped_assets_delegated_from,
        api::group::get_asset_group_by_id,
        api::group::register_asset_group,
        api::group::update_asset_group,
        api::group::remove_asset_group,
    ),
    components(
        schemas(
            vehicle::Data,
            vertiport::Data,
            vertipad::Data,
            GeoPoint,
            GeoPolygon,
            GeoLineString,
            RegisterAssetGroupPayload,
            UpdateAircraftPayload,
            UpdateVertiportPayload,
            UpdateVertipadPayload,
            structs::Operator,
            structs::Aircraft,
            structs::Vertiport,
            structs::Vertipad,
            structs::AssetGroup,
            structs::AssetStatus,
            structs::Basics,
        )
    ),
    tags(
        (name = "svc-assets", description = "svc-assets API")
    )
)]
struct ApiDoc;

/// Create OpenAPI3 Specification File
pub fn generate_openapi_spec(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = ApiDoc::openapi()
        .to_pretty_json()
        .expect("(ERROR) unable to write openapi specification to json.");

    std::fs::write(target, output).expect("(ERROR) unable to write json string to file.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_spec_generation() {
        assert!(generate_openapi_spec("/tmp/generate_openapi_spec.out").is_ok());
    }
}
