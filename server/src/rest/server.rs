//! Rest server implementation

use super::api;
use crate::grpc::client::GrpcClients;
use crate::shutdown_signal;
use crate::Config;
use axum::{
    error_handling::HandleErrorLayer,
    extract::Extension,
    http::{HeaderValue, StatusCode},
    routing, BoxError, Router,
};
use std::net::SocketAddr;
use tower::{
    buffer::BufferLayer,
    limit::{ConcurrencyLimitLayer, RateLimitLayer},
    ServiceBuilder,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

/// Starts the REST API server for this microservice
///
/// # Example:
/// ```
/// use svc_assets::rest::server::rest_server;
/// use svc_assets::Config;
/// async fn example() -> Result<(), tokio::task::JoinError> {
///     let config = Config::default();
///     tokio::spawn(rest_server(config, None)).await;
///     Ok(())
/// }
/// ```
pub async fn rest_server(
    config: Config,
    shutdown_rx: Option<tokio::sync::oneshot::Receiver<()>>,
) -> Result<(), ()> {
    rest_info!("entry.");
    let rest_port = config.docker_port_rest;

    let full_rest_addr: SocketAddr = format!("[::]:{}", rest_port).parse().map_err(|e| {
        rest_error!("invalid address: {:?}, exiting.", e);
    })?;

    let cors_allowed_origin = config
        .rest_cors_allowed_origin
        .parse::<HeaderValue>()
        .map_err(|e| {
            rest_error!("invalid cors_allowed_origin address: {:?}, exiting.", e);
        })?;

    // Rate limiting
    let rate_limit = config.rest_request_limit_per_second as u64;
    let concurrency_limit = config.rest_concurrency_limit_per_service as usize;
    let limit_middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(HandleErrorLayer::new(|e: BoxError| async move {
            rest_warn!("too many requests: {}", e);
            (
                StatusCode::TOO_MANY_REQUESTS,
                "(server) too many requests.".to_string(),
            )
        }))
        .layer(BufferLayer::new(100))
        .layer(ConcurrencyLimitLayer::new(concurrency_limit))
        .layer(RateLimitLayer::new(
            rate_limit,
            std::time::Duration::from_secs(1),
        ));

    //
    // Extensions
    //
    // GRPC Clients
    let grpc_clients = GrpcClients::default(config.clone());

    let app = Router::new()
        .route("/health", routing::get(api::health::health_check))
        .route(
            "/assets/operators/:id",
            routing::get(api::operator::get_operator),
        )
        .route(
            "/assets/demo/aircraft",
            routing::get(api::aircraft::get_all_aircraft),
        )
        .route(
            "/assets/demo/vertiports",
            routing::get(api::vertiport::get_all_vertiports),
        )
        .route(
            "/assets/demo/vertipads",
            routing::get(api::vertipad::get_all_vertipads),
        )
        .route(
            "/assets/operators/:id/assets",
            routing::get(api::operator::get_all_assets_by_operator),
        )
        .route(
            "/assets/operators/:id/grouped",
            routing::get(api::operator::get_all_grouped_assets),
        )
        .route(
            "/assets/operators/:id/grouped/delegated-to",
            routing::get(api::operator::get_all_grouped_assets_delegated_to),
        )
        .route(
            "/assets/operators/:id/grouped/delegated-from",
            routing::get(api::operator::get_all_grouped_assets_delegated_from),
        )
        .route(
            "/assets/aircraft/:id",
            routing::get(api::aircraft::get_aircraft_by_id),
        )
        .route(
            "/assets/vertipads/:id",
            routing::get(api::vertipad::get_vertipad_by_id),
        )
        .route(
            "/assets/vertiports/:id",
            routing::get(api::vertiport::get_vertiport_by_id),
        )
        .route(
            "/assets/groups/:id",
            routing::get(api::group::get_asset_group_by_id),
        )
        // POST endpoints
        .route(
            "/assets/aircraft",
            routing::post(api::aircraft::register_aircraft),
        )
        .route(
            "/assets/vertiports",
            routing::post(api::vertiport::register_vertiport),
        )
        .route(
            "/assets/vertipads",
            routing::post(api::vertipad::register_vertipad),
        )
        .route(
            "/assets/groups",
            routing::post(api::group::register_asset_group),
        )
        // PUT endpoints
        .route(
            "/assets/aircraft",
            routing::put(api::aircraft::update_aircraft),
        )
        .route(
            "/assets/vertiports",
            routing::put(api::vertiport::update_vertiport),
        )
        .route(
            "/assets/vertipads",
            routing::put(api::vertipad::update_vertipad),
        )
        .route(
            "/assets/groups/:id",
            routing::put(api::group::update_asset_group),
        )
        // DELETE endpoints
        .route(
            "/assets/aircraft/:id",
            routing::delete(api::aircraft::remove_aircraft),
        )
        .route(
            "/assets/vertiports/:id",
            routing::delete(api::vertiport::remove_vertiport),
        )
        .route(
            "/assets/vertipads/:id",
            routing::delete(api::vertipad::remove_vertipad),
        )
        .route(
            "/assets/groups/:id",
            routing::delete(api::group::remove_asset_group),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(cors_allowed_origin)
                .allow_headers(Any)
                .allow_methods(Any),
        )
        .layer(limit_middleware)
        .layer(Extension(grpc_clients)); // Extension layer must be last

    //
    // Bind to address
    //
    axum::Server::bind(&full_rest_addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal("rest", shutdown_rx))
        .await
        .map_err(|e| {
            rest_error!("could not start server: {}", e);
        })?;

    rest_info!("server running at: {}.", full_rest_addr);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_start_and_shutdown() {
        use tokio::time::{sleep, Duration};
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        let config = Config::default();

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

        // Start the rest server
        tokio::spawn(rest_server(config, Some(shutdown_rx)));

        // Give the server time to get through the startup sequence (and thus code)
        sleep(Duration::from_secs(1)).await;

        // Shut down server
        assert!(shutdown_tx.send(()).is_ok());

        ut_info!("success");
    }
}
