//! REST
//! provides server implementations for REST API

#[macro_use]
pub mod macros;
pub mod api;
pub mod server;
pub mod structs;

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
            api::rest_types::RegisterAircraftPayload,
            api::rest_types::RegisterVertiportPayload,
            api::rest_types::RegisterVertipadPayload,
            api::rest_types::RegisterAssetGroupPayload,
            api::rest_types::UpdateAircraftPayload,
            api::rest_types::UpdateVertiportPayload,
            api::rest_types::UpdateVertipadPayload,
            structs::Operator,
            structs::Aircraft,
            structs::Vertiport,
            structs::Vertipad,
            structs::AssetGroup,
            structs::AssetStatus,
            structs::Basics,
            structs::Location,
            structs::OrderedFloat64,
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
