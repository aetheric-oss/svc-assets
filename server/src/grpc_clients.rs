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
    pub storage_vertiport: GrpcClient<VertiportRpcClient<Channel>>,
    pub storage_vertipad: GrpcClient<VertipadRpcClient<Channel>>,
    pub storage_vehicle: GrpcClient<VehicleRpcClient<Channel>>,
}

#[derive(Debug, Clone)]
pub struct GrpcClient<T> {
    inner: Arc<Mutex<Option<T>>>,
    address: String,
}

/// Returns a string in http://host:port format from provided
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

macro_rules! grpc_client {
    ( $client: ident, $name: expr ) => {
        impl GrpcClient<$client<Channel>> {
            pub async fn get_client(&mut self) -> Option<$client<Channel>> {
                grpc_debug!("(get_client) storage::{} entry", $name);

                let arc = Arc::clone(&self.inner);

                // if already connected, return the client
                let client = arc.lock().await;
                if client.is_some() {
                    return client.clone();
                }

                grpc_debug!(
                    "(grpc) connecting to {} server at {}",
                    $name,
                    self.address.clone()
                );
                let result = $client::connect(self.address.clone()).await;
                match result {
                    Ok(client) => {
                        grpc_info!(
                            "(grpc) success: connected to {} server at {}",
                            $name,
                            self.address.clone()
                        );
                        Some(client)
                    }
                    Err(e) => {
                        grpc_error!(
                            "(grpc) couldn't connect to {} server at {}; {}",
                            $name,
                            self.address,
                            e
                        );
                        None
                    }
                }
            }
        }
    };
}

grpc_client!(VehicleRpcClient, "aircraft");
grpc_client!(VertipadRpcClient, "vertipad");
grpc_client!(VertiportRpcClient, "vertiport");

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
