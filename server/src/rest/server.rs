//! Rest server implementation

use super::api;
use crate::config::Config;
use crate::grpc::client::GrpcClients;
use crate::shutdown_signal;
use axum::{extract::Extension, routing, Router};

/// Starts the REST API server for this microservice
pub async fn rest_server(config: Config) {
    use std::net::SocketAddr;
    let grpc_clients = GrpcClients::default();

    rest_debug!("(rest_server) entry.");
    let rest_port = config.docker_port_rest;

    let full_rest_addr: SocketAddr = match format!("[::]:{}", rest_port).parse() {
        Ok(addr) => addr,
        Err(e) => {
            rest_error!("Failed to parse REST address: {}", e);
            return;
        }
    };

    let app = Router::new()
        // .merge(SwaggerUi::new("/swagger-ui/*tail").url("/api-doc/openapi.json", ApiDoc::openapi()))
        // GET endpoints
        .route("/assets/operators/:id", routing::get(api::get_operator))
        .route("/assets/demo/aircraft", routing::get(api::get_all_aircraft))
        .route(
            "/assets/demo/vertiports",
            routing::get(api::get_all_vertiports),
        )
        .route(
            "/assets/demo/vertipads",
            routing::get(api::get_all_vertipads),
        )
        .route(
            "/assets/operators/:id/assets",
            routing::get(api::get_all_assets_by_operator),
        )
        .route(
            "/assets/operators/:id/grouped",
            routing::get(api::get_all_grouped_assets),
        )
        .route(
            "/assets/operators/:id/grouped/delegated-to",
            routing::get(api::get_all_grouped_assets_delegated_to),
        )
        .route(
            "/assets/operators/:id/grouped/delegated-from",
            routing::get(api::get_all_grouped_assets_delegated_from),
        )
        .route(
            "/assets/aircraft/:id",
            routing::get(api::get_aircraft_by_id),
        )
        .route(
            "/assets/vertipads/:id",
            routing::get(api::get_vertipad_by_id),
        )
        .route(
            "/assets/vertiports/:id",
            routing::get(api::get_vertiport_by_id),
        )
        .route(
            "/assets/groups/:id",
            routing::get(api::get_asset_group_by_id),
        )
        // POST endpoints
        .route("/assets/aircraft", routing::post(api::register_aircraft))
        .route("/assets/vertiports", routing::post(api::register_vertiport))
        .route("/assets/vertipads", routing::post(api::register_vertipad))
        .route("/assets/groups", routing::post(api::register_asset_group))
        // PUT endpoints
        .route("/assets/aircraft", routing::put(api::update_aircraft))
        .route("/assets/vertiports", routing::put(api::update_vertiport))
        .route("/assets/vertipads", routing::put(api::update_vertipad))
        .route("/assets/groups/:id", routing::put(api::update_asset_group))
        // DELETE endpoints
        .route(
            "/assets/aircraft/:id",
            routing::delete(api::remove_aircraft),
        )
        .route(
            "/assets/vertiports/:id",
            routing::delete(api::remove_vertiport),
        )
        .route(
            "/assets/vertipads/:id",
            routing::delete(api::remove_vertipad),
        )
        .route(
            "/assets/groups/:id",
            routing::delete(api::remove_asset_group),
        )
        .layer(Extension(grpc_clients)); // Extension layer must be last

    rest_info!("(rest) hosted at {:?}", full_rest_addr);
    match axum::Server::bind(&full_rest_addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal("rest"))
        .await
    {
        Ok(_) => rest_info!("REST server running at: {}.", full_rest_addr),
        Err(e) => {
            rest_error!("could not start REST server: {}", e);
        }
    };
}
