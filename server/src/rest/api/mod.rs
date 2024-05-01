//! Handlers for the REST API

/// Types used in REST messages to this server
pub mod rest_types {
    include!("../../../../openapi/types.rs");
}

pub mod aircraft;
pub mod group;
pub mod health;
pub mod operator;
pub mod vertipad;
pub mod vertiport;
