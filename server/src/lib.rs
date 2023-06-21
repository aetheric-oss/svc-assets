#![doc = include_str!("../README.md")]

mod config;
pub use crate::config::Config;
pub mod grpc;

pub use clap::Parser;
use log::warn;

pub mod rest;

/// struct holding cli configuration options
#[derive(Parser, Debug)]
pub struct Cli {
    /// Target file to write the OpenAPI Spec
    #[arg(long)]
    pub openapi: Option<String>,
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// This signal handler can be used in our [`axum::Server`] method `with_graceful_shutdown`
/// and in our [`tonic::transport::Server`] method `serve_with_shutdown`.
///
/// # Examples
///
/// ## axum
/// ```
/// use svc_assets::shutdown_signal;
/// pub async fn server() {
///     let app = axum::Router::new();
///     axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
///         .serve(app.into_make_service())
///         .with_graceful_shutdown(shutdown_signal("rest", None));
/// }
/// ```
///
/// ## tonic
/// ```
/// use svc_assets::shutdown_signal;
/// pub async fn server() {
///     let (_, health_service) = tonic_health::server::health_reporter();
///     tonic::transport::Server::builder()
///         .add_service(health_service)
///         .serve_with_shutdown("0.0.0.0:50051".parse().unwrap(), shutdown_signal("grpc", None));
/// }
/// ```
///
/// ## using a shutdown signal channel
/// ```
/// use svc_assets::shutdown_signal;
/// pub async fn server() {
///     let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
///     let (_, health_service) = tonic_health::server::health_reporter();
///     tokio::spawn(async move {
///         tonic::transport::Server::builder()
///             .add_service(health_service)
///             .serve_with_shutdown("0.0.0.0:50051".parse().unwrap(), shutdown_signal("grpc", Some(shutdown_rx)))
///             .await;
///     });
///
///     // Send server the shutdown request
///     shutdown_tx.send(()).expect("Could not stop server.");
/// }
/// ```
pub async fn shutdown_signal(
    server: &str,
    shutdown_rx: Option<tokio::sync::oneshot::Receiver<()>>,
) {
    match shutdown_rx {
        Some(receiver) => receiver
            .await
            .expect("(shutdown_signal) expect tokio signal oneshot Receiver"),
        None => tokio::signal::ctrl_c()
            .await
            .expect("(shutdown_signal) expect tokio signal ctrl-c"),
    }

    warn!("(shutdown_signal) server shutdown for [{}]", server);
}
