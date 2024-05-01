use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use lib_common::time::{DateTime, Utc};

pub use svc_storage_client_grpc::prelude::{GeoLineString, GeoPoint, GeoPolygon};
pub use svc_storage_client_grpc::resources::{vehicle, vertipad, vertiport};

/// Status of an Asset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub enum AssetStatus {
    /// The asset is available for use.
    Available,
    /// The asset is unavailable for use.
    Unavailable,
    /// The asset is only available for emergencies.
    Emergency,
}

/// Request to create an Asset Group.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RegisterAssetGroupPayload {
    /// Optional identification name of the asset group
    pub name: Option<String>,
    /// The UUID of an Operator, if available.
    pub owner: String,
    /// A list of UUIDs of assets.
    pub assets: Vec<String>,
}

/// Request to update an Aircraft.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct UpdateAircraftPayload {
    /// The UUID v4 of the Aircraft to update.
    pub id: String,
    /// The vehicle_model_id UUID v4, can be used to collect additional vehicle_model information.
    pub vehicle_model_id: Option<String>,
    /// optional id UUID v4 of the hangar (vertipad) this aircraft is assigned to
    pub hangar_id: Option<String>,
    /// optional id UUID v4 of the hangar bay (vertiport) this aircraft is assigned to
    pub hangar_bay_id: Option<String>,
    /// The Aircraft's unique serial_number given at the factory.
    pub serial_number: Option<String>,
    /// The Aircraft's registration number.
    ///
    /// In the US, this is the N number.
    ///
    /// This is a unique identifier for
    /// the aircraft that can be used to look up information about the
    /// aircraft from national aviation authorities like the FAA.
    pub registration_number: Option<String>,
    /// Optional additional description of the Aircraft.
    pub description: Option<String>,
    /// The UUID of an AssetGroup, if available.
    pub asset_group_id: Option<String>,
    /// Optional RRULE data string to indicate the Aircraft's available days and hours.
    pub schedule: Option<String>,
    /// Optional date of Aircraft's last maintenance.
    pub last_maintenance: Option<DateTime<Utc>>,
    /// Optional date of Aircraft's next planned maintenance.
    pub next_maintenance: Option<DateTime<Utc>>,
    /// List of fields that should be updated.
    ///
    /// If any other fields are provided, they will be ignored.
    pub mask: Vec<String>,
}

/// Request to update a Vertiport.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct UpdateVertiportPayload {
    /// The UUID v4 of the Vertiport to update.
    pub id: String,
    /// Identification name of the Vertiport.
    pub name: Option<String>,
    /// Additional description of the Vertiport.
    pub description: Option<String>,
    /// Geographical area location of the Vertiport.
    pub geo_location: Option<GeoPolygon>,
    /// Optional RRULE data string to indicate the Vertiport's available days and hours.
    pub schedule: Option<String>,
    /// List of fields that should be updated.
    ///
    /// If any other fields are provided, they will be ignored.
    pub mask: Vec<String>,
}

/// Request to update a Vertipad.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct UpdateVertipadPayload {
    /// The UUID v4 of the Vertipad to update.
    pub id: String,
    /// The UUID v4 of the Vertiport the Vertipad is located at.
    pub vertiport_id: Option<String>,
    /// Identification name of the Vertipad.
    pub name: Option<String>,
    /// Geographical location of the Vertipad.
    pub geo_location: Option<GeoPoint>,
    /// Indicates if the Vertipad is in business.
    pub enabled: Option<bool>,
    /// Indicates if the Vertipad is currently occupied.
    pub occupied: Option<bool>,
    /// Optional RRULE data string to indicate the Vertipad's available days and hours.
    pub schedule: Option<String>,
    /// List of fields that should be updated.
    ///
    /// If any other fields are provided, they will be ignored.
    pub mask: Vec<String>,
}
