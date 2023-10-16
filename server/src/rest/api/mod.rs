//! REST API Handlers for the server

/// Types Used in REST Messages
pub mod rest_types {
    include!("../../../../openapi/types.rs"); // TODO(R4): Use Cargo manifest directory
}

/// Check if a string is a valid UUID.
fn is_uuid(s: &str) -> bool {
    uuid::Uuid::try_parse(s).is_ok()
}

pub mod asset;
pub mod group;
pub mod health;
pub mod operator;
pub mod structs;
