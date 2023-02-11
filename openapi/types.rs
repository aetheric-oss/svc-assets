use crate::structs::{AssetStatus, Location};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

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
    pub max_payload_kg: f64,
    pub max_range_km: f64,
    pub last_maintenance: Option<String>,
    pub next_maintenance: Option<String>,
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
    pub name: Option<String>,
    pub vertiport_id: String,
    pub status: AssetStatus,
    pub location: Location,
    pub enabled: bool,
    pub occupied: bool,
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

/// Request to update an aircraft.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct UpdateAircraftPayload {
    pub id: String,
    /// The UUID of the model.
    pub vehicle_model_id: Option<String>,
    pub last_vertiport_id: Option<Option<String>>,
    pub serial_number: Option<String>,
    pub registration_number: Option<String>,
    pub description: Option<Option<String>>,
    pub asset_group_id: Option<Option<String>>,
    pub schedule: Option<Option<String>>,
    pub last_maintenance: Option<Option<String>>,
    pub next_maintenance: Option<Option<String>>,
    pub mask: Vec<String>,
}

/// Request to update a vertiport.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct UpdateVertiportPayload {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub schedule: Option<Option<String>>,
    pub mask: Vec<String>,
}

/// Request to update a vertipad.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct UpdateVertipadPayload {
    pub id: String,
    pub vertiport_id: Option<String>,
    pub name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub enabled: Option<bool>,
    pub occupied: Option<bool>,
    pub schedule: Option<Option<String>>,
    pub mask: Vec<String>,
}
