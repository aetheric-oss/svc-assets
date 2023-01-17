use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::structs::{AssetStatus, Location};

/// Request to create an aircraft.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RegisterAircraftPayload {
    pub name: Option<String>,
    /// The UUID of an [`AssetGroup`] struct, if available.
    pub group_id: Option<String>,
    pub owner: String,
    pub whitelist: Vec<String>,
    pub status: AssetStatus,
    /// The aircraft's manufacturer.
    ///
    /// TODO: For now we can just say "Boeing", "Airbus", etc. Later, we
    /// can a struct for this and store the manufacturer's name, logo,
    /// etc.
    pub manufacturer: String,
    pub model: String,
    pub serial_number: String,
    /// The aircraft's registration number.
    ///
    /// In the US, this is the N number.
    ///
    /// This is a unique identifier for
    /// the aircraft that can be used to look up information about the
    /// aircraft from national aviation authorities like the FAA.
    pub registration_number: String,
    pub description: Option<String>,
    pub max_payload_kg: OrderedFloat<f32>,
    pub max_range_km: Option<OrderedFloat<f64>>,
}

/// Request to create a vertiport.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RegisterVertiportPayload {
    pub name: Option<String>,
    /// The UUID of an [`AssetGroup`] struct, if available.
    pub group_id: Option<String>,
    pub owner: String,
    pub whitelist: Vec<String>,
    pub status: AssetStatus,
    pub description: Option<String>,
    pub location: Location,
}

/// Request to create a vertipad.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RegisterVertipadPayload {
    pub vertiport_id: String,
    pub status: AssetStatus,
    pub location: Location,
}

/// Request to create an asset group.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RegisterAssetGroupPayload {
    pub name: Option<String>,
    /// The UUID of an [`Operator`] struct, if available.
    pub owner: String,
    /// A list of UUIDs of assets.
    pub assets: Vec<String>,
}
