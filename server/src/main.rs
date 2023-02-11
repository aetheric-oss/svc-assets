//! gRPC server implementation

///module svc_assets generated from svc-assets-grpc.proto
pub mod svc_assets {
    #![allow(unused_qualifications, missing_docs)]
    include!("grpc.rs");
}

mod rest_api;
//TODO - replace this with database calls when ready
mod grpc_clients;
mod structs;
#[macro_use]
mod loggers;

use axum::{extract::Extension, handler::Handler, response::IntoResponse, routing, Router};
use clap::Parser;
use grpc_clients::GrpcClients;
use log::{info, warn};
use structs::*;
use svc_assets::svc_assets_rpc_server::{SvcAssetsRpc, SvcAssetsRpcServer};
use svc_assets::{QueryIsReady, ReadyResponse};
use tonic::{Request, Response, Status};
use utoipa::OpenApi;

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct SvcAssetsImpl {}

#[derive(Parser, Debug)]
struct Cli {
    /// Target file to write the OpenAPI Spec
    #[arg(long)]
    openapi: Option<String>,
}

#[tonic::async_trait]
impl SvcAssetsRpc for SvcAssetsImpl {
    /// Returns ready:true when service is available
    async fn is_ready(
        &self,
        _request: Request<QueryIsReady>,
    ) -> Result<Response<ReadyResponse>, Status> {
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }
}

/// Starts the grpc server for this microservice
async fn grpc_server() {
    // GRPC Server
    let grpc_port = std::env::var("DOCKER_PORT_GRPC")
        .unwrap_or_else(|_| "50051".to_string())
        .parse::<u16>()
        .unwrap_or(50051);

    let addr = format!("[::]:{grpc_port}").parse().unwrap();
    let imp = SvcAssetsImpl::default();
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<SvcAssetsRpcServer<SvcAssetsImpl>>()
        .await;

    info!("(grpc) hosted at {}", addr);
    tonic::transport::Server::builder()
        .add_service(health_service)
        .add_service(SvcAssetsRpcServer::new(imp))
        .serve_with_shutdown(addr, shutdown_signal("grpc"))
        .await
        .unwrap();
}

//-----------------------------------------------------------
// REST Server
//-----------------------------------------------------------

/// Responds a NOT_FOUND status and error string
///
/// # Examples
///
/// ```
/// let app = Router::new()
///         .fallback(not_found.into_service());
/// ```
pub async fn not_found(uri: axum::http::Uri) -> impl IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
///
/// # Examples
///
/// ```
/// Server::bind(&"0.0.0.0:8000".parse().unwrap())
/// .serve(app.into_make_service())
/// .with_graceful_shutdown(shutdown_signal())
/// .await
/// .unwrap();
/// ```
pub async fn shutdown_signal(server: &str) {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    warn!("({}) shutdown signal", server);
}

#[derive(OpenApi)]
#[openapi(
    paths(
        rest_api::get_operator,
        rest_api::get_all_assets,
        rest_api::get_all_grouped_assets,
        rest_api::get_all_grouped_assets_delegated_to,
        rest_api::get_all_grouped_assets_delegated_from,
        rest_api::get_aircraft_by_id,
        rest_api::get_vertipad_by_id,
        rest_api::get_vertiport_by_id,
        rest_api::get_asset_group_by_id,
        rest_api::register_aircraft,
        rest_api::register_vertiport,
        rest_api::register_vertipad,
        rest_api::register_asset_group,
        rest_api::update_aircraft,
        rest_api::update_vertiport,
        rest_api::update_vertipad,
        rest_api::update_asset_group,
        rest_api::remove_aircraft,
        rest_api::remove_vertiport,
        rest_api::remove_vertipad,
        rest_api::remove_asset_group,
    ),
    components(
        schemas(
            rest_api::rest_types::RegisterAircraftPayload,
            rest_api::rest_types::RegisterVertiportPayload,
            rest_api::rest_types::RegisterVertipadPayload,
            rest_api::rest_types::RegisterAssetGroupPayload,
            Aircraft,
            Vertiport,
            Vertipad,
            AssetGroup,
            AssetStatus,
            Basics,
            Location,
            OrderedFloat64,
        )
    ),
    tags(
        (name = "svc-assets", description = "svc-assets API")
    )
)]
struct ApiDoc;

/// Starts the REST API server for this microservice
pub async fn rest_server(grpc_clients: GrpcClients) {
    let rest_port = std::env::var("DOCKER_PORT_REST")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .unwrap_or(8000);

    let app = Router::new()
        // .merge(SwaggerUi::new("/swagger-ui/*tail").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .fallback(not_found.into_service())
        // GET endpoints
        .route(
            "/assets/operators/:id",
            routing::get(rest_api::get_operator),
        )
        .route(
            "/assets/operators/:id/assets",
            routing::get(rest_api::get_all_assets),
        )
        .route(
            "/assets/operators/:id/grouped",
            routing::get(rest_api::get_all_grouped_assets),
        )
        .route(
            "/assets/operators/:id/grouped/delegated-to",
            routing::get(rest_api::get_all_grouped_assets_delegated_to),
        )
        .route(
            "/assets/operators/:id/grouped/delegated-from",
            routing::get(rest_api::get_all_grouped_assets_delegated_from),
        )
        .route(
            "/assets/aircraft/:id",
            routing::get(rest_api::get_aircraft_by_id),
        )
        .route(
            "/assets/vertipads/:id",
            routing::get(rest_api::get_vertipad_by_id),
        )
        .route(
            "/assets/vertiports/:id",
            routing::get(rest_api::get_vertiport_by_id),
        )
        .route(
            "/assets/groups/:id",
            routing::get(rest_api::get_asset_group_by_id),
        )
        // POST endpoints
        .route(
            "/assets/aircraft",
            routing::post(rest_api::register_aircraft),
        )
        .route(
            "/assets/vertiports",
            routing::post(rest_api::register_vertiport),
        )
        .route(
            "/assets/vertipads",
            routing::post(rest_api::register_vertipad),
        )
        .route(
            "/assets/groups",
            routing::post(rest_api::register_asset_group),
        )
        // PUT endpoints
        .route(
            "/assets/aircraft/:id",
            routing::put(rest_api::update_aircraft),
        )
        .route(
            "/assets/vertiports/:id",
            routing::put(rest_api::update_vertiport),
        )
        .route(
            "/assets/vertipads/:id",
            routing::put(rest_api::update_vertipad),
        )
        .route(
            "/assets/groups/:id",
            routing::put(rest_api::update_asset_group),
        )
        // DELETE endpoints
        .route(
            "/assets/aircraft/:id",
            routing::delete(rest_api::remove_aircraft),
        )
        .route(
            "/assets/vertiports/:id",
            routing::delete(rest_api::remove_vertiport),
        )
        .route(
            "/assets/vertipads/:id",
            routing::delete(rest_api::remove_vertipad),
        )
        .route(
            "/assets/groups/:id",
            routing::delete(rest_api::remove_asset_group),
        )
        .layer(Extension(grpc_clients)); // Extension layer must be last

    let address = format!("[::]:{rest_port}").parse().unwrap();
    info!("(rest) hosted at {:?}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal("rest"))
        .await
        .unwrap();
}

/// Create OpenAPI3 Specification File
fn generate_openapi_spec(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = ApiDoc::openapi()
        .to_pretty_json()
        .expect("(ERROR) unable to write openapi specification to json.");

    std::fs::write(target, output).expect("(ERROR) unable to write json string to file.");

    Ok(())
}

///Main entry point: starts gRPC Server on specified address and port
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Allow option to only generate the spec file to a given location
    let args = Cli::parse();
    if let Some(target) = args.openapi {
        return generate_openapi_spec(&target);
    }

    // Initialize logger
    {
        let log_cfg: &str = "log4rs.yaml";
        if let Err(e) = log4rs::init_file(log_cfg, Default::default()) {
            println!("(logger) could not parse {}. {}", log_cfg, e);
            panic!();
        }
    }

    // Start GRPC Server
    tokio::spawn(grpc_server());

    // Wait for other GRPC Servers
    let grpc_clients = GrpcClients::default();

    // Start REST API
    rest_server(grpc_clients).await;

    info!("Successful shutdown.");

    Ok(())
}
