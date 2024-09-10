//! Re-export of used objects

pub use super::client as assets;
pub use super::service::Client as AssetsServiceClient;
pub use assets::AssetsClient;

pub use lib_common::grpc::Client;
