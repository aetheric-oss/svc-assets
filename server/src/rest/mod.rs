//! REST
//! provides server implementations for REST API

#[macro_use]
pub mod macros;
pub mod api;
pub mod server;
pub mod structs;

use api::*;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::get_operator,
        api::get_all_aircraft,
        api::get_all_vertiports,
        api::get_all_vertipads,
        api::get_all_assets_by_operator,
        api::get_all_grouped_assets,
        api::get_all_grouped_assets_delegated_to,
        api::get_all_grouped_assets_delegated_from,
        api::get_aircraft_by_id,
        api::get_vertipad_by_id,
        api::get_vertiport_by_id,
        api::get_asset_group_by_id,
        api::register_aircraft,
        api::register_vertiport,
        api::register_vertipad,
        api::register_asset_group,
        api::update_aircraft,
        api::update_vertiport,
        api::update_vertipad,
        api::update_asset_group,
        api::remove_aircraft,
        api::remove_vertiport,
        api::remove_vertipad,
        api::remove_asset_group,
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
