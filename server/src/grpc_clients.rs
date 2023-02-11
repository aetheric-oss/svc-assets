//! This module contains the GrpcClients struct which is used to
//! manage the gRPC clients for the various microservices.

pub use svc_storage_client_grpc::{
    VehicleClient as VehicleRpcClient, VertipadClient as VertipadRpcClient,
    VertiportClient as VertiportRpcClient,
};

use futures::lock::Mutex;
use log::{debug, error};
use std::sync::Arc;
pub use tonic::transport::Channel;

use crate::{grpc_debug, grpc_error, grpc_info};

#[derive(Clone, Debug)]
pub struct GrpcClients {
    //TODO
    pub storage_vertiport: GrpcClient<VertiportRpcClient<Channel>>,
    pub storage_vertipad: GrpcClient<VertipadRpcClient<Channel>>,
    pub storage_vehicle: GrpcClient<VehicleRpcClient<Channel>>,
}

#[derive(Debug, Clone)]
pub struct GrpcClient<T> {
    inner: Arc<Mutex<Option<T>>>,
    address: String,
}

/// Returns a string in http://<host>:<port> format from provided
/// environment variables
fn get_grpc_endpoint(env_host: &str, env_port: &str) -> String {
    debug!("(get_grpc_endpoint) entry");
    let port = match std::env::var(env_port) {
        Ok(s) => s,
        Err(_) => {
            error!("(env) {} undefined.", env_port);
            "".to_string()
        }
    };
    let host = match std::env::var(env_host) {
        Ok(s) => s,
        Err(_) => {
            error!("(env) {} undefined.", env_host);
            "".to_string()
        }
    };

    format!("http://{host}:{port}")
}

impl<T> GrpcClient<T> {
    pub async fn invalidate(&mut self) {
        let arc = Arc::clone(&self.inner);
        let mut client = arc.lock().await;
        *client = None;
    }

    pub fn new(env_host: &str, env_port: &str) -> Self {
        let opt: Option<T> = None;
        GrpcClient {
            inner: Arc::new(Mutex::new(opt)),
            address: get_grpc_endpoint(env_host, env_port),
        }
    }
}

// TODO Figure out how to collapse these three implementations for each client into
//   one generic impl. VertiportRpcClient does not simply impl a trait,
//   it wraps the tonic::client::Grpc<T> type so it's a bit tricky
impl GrpcClient<VertiportRpcClient<Channel>> {
    pub async fn get_client(&mut self) -> Option<VertiportRpcClient<Channel>> {
        grpc_debug!("(get_client) storage::vertiport entry");

        let arc = Arc::clone(&self.inner);
        let mut client = arc.lock().await;

        if client.is_none() {
            grpc_info!(
                "(grpc) connecting to svc-storage vertiport server at {}",
                self.address.clone()
            );
            let client_option = match VertiportRpcClient::connect(self.address.clone()).await {
                Ok(s) => Some(s),
                Err(e) => {
                    grpc_error!(
                        "(grpc) couldn't connect to svc-storage vertiport server at {}; {}",
                        self.address,
                        e
                    );
                    None
                }
            };

            *client = client_option;
        }

        client.clone()
    }
}

impl GrpcClient<VertipadRpcClient<Channel>> {
    pub async fn get_client(&mut self) -> Option<VertipadRpcClient<Channel>> {
        grpc_debug!("(get_client) storage::vertiport entry");

        let arc = Arc::clone(&self.inner);
        let mut client = arc.lock().await;

        if client.is_none() {
            grpc_info!(
                "(grpc) connecting to svc-storage vertiport server at {}",
                self.address.clone()
            );
            let client_option = match VertipadRpcClient::connect(self.address.clone()).await {
                Ok(s) => Some(s),
                Err(e) => {
                    grpc_error!(
                        "(grpc) couldn't connect to svc-storage vertiport server at {}; {}",
                        self.address,
                        e
                    );
                    None
                }
            };

            *client = client_option;
        }

        client.clone()
    }
}

impl GrpcClient<VehicleRpcClient<Channel>> {
    pub async fn get_client(&mut self) -> Option<VehicleRpcClient<Channel>> {
        grpc_debug!("(get_client) storage::vertiport entry");

        let arc = Arc::clone(&self.inner);
        let mut client = arc.lock().await;

        if client.is_none() {
            grpc_info!(
                "(grpc) connecting to svc-storage vertiport server at {}",
                self.address.clone()
            );
            let client_option = match VehicleRpcClient::connect(self.address.clone()).await {
                Ok(s) => Some(s),
                Err(e) => {
                    grpc_error!(
                        "(grpc) couldn't connect to svc-storage vertiport server at {}; {}",
                        self.address,
                        e
                    );
                    None
                }
            };

            *client = client_option;
        }

        client.clone()
    }
}

impl GrpcClients {
    pub fn default() -> Self {
        GrpcClients {
            storage_vertiport: GrpcClient::<VertiportRpcClient<Channel>>::new(
                "STORAGE_HOST_GRPC",
                "STORAGE_PORT_GRPC",
            ),
            storage_vertipad: GrpcClient::<VertipadRpcClient<Channel>>::new(
                "STORAGE_HOST_GRPC",
                "STORAGE_PORT_GRPC",
            ),
            storage_vehicle: GrpcClient::<VehicleRpcClient<Channel>>::new(
                "STORAGE_HOST_GRPC",
                "STORAGE_PORT_GRPC",
            ),
        }
    }
}
