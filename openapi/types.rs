use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// A wrapper for `OrderedFloat<f64>` for documentation generation purposes.
#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Copy)]
pub struct OrderedFloat64(pub OrderedFloat<f64>);
#[allow(missing_docs)]
impl From<f64> for OrderedFloat64 {
    fn from(value: f64) -> Self {
        OrderedFloat64(OrderedFloat(value))
    }
}

impl OrderedFloat64 {
    /// Convert the value to a f64.
    pub fn to_f64(self) -> f64 {
        self.0.into_inner()
    }
}

#[allow(missing_docs)]
impl ToSchema for OrderedFloat64 {
    fn schema() -> utoipa::openapi::schema::Schema {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                "value",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Number)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Float,
                    ))),
            )
            .required("value")
            .into()
    }
}

#[allow(missing_docs, missing_copy_implementations)]
/// Status of an asset.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub enum AssetStatus {
    /// The asset is available for use.
    Available,
    /// The asset is unavailable for use.
    Unavailable,
    /// The asset is only available for emergencies.
    Emergency,
}

/// A struct representing a location.
#[allow(missing_docs, missing_copy_implementations)]
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Location {
    pub latitude: OrderedFloat64,
    pub longitude: OrderedFloat64,
}

/// Request to create an aircraft.
#[allow(missing_docs, missing_copy_implementations)]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RegisterAircraftPayload {
    pub name: Option<String>,
    /// The UUID of an [`crate::structs::AssetGroup`] struct, if available.
    pub group_id: Option<String>,
    pub owner: String,
    pub whitelist: Vec<String>,
    pub status: AssetStatus,
    /// The aircraft's manufacturer.
    ///
    /// TODO R3/4: For now we can just say "Boeing", "Airbus", etc. Later, we
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
    pub last_vertiport_id: Option<String>,
}

/// Request to create a vertiport.
#[allow(missing_docs, missing_copy_implementations)]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RegisterVertiportPayload {
    pub name: Option<String>,
    /// The UUID of an [`crate::structs::AssetGroup`] struct, if available.
    pub group_id: Option<String>,
    pub owner: String,
    pub whitelist: Vec<String>,
    pub status: AssetStatus,
    pub description: Option<String>,
    pub location: Location,
}

/// Request to create a vertipad.
#[allow(missing_docs, missing_copy_implementations)]
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
#[allow(missing_docs, missing_copy_implementations)]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RegisterAssetGroupPayload {
    pub name: Option<String>,
    /// The UUID of an [`crate::structs::Operator`] struct, if available.
    pub owner: String,
    /// A list of UUIDs of assets.
    pub assets: Vec<String>,
}

/// Request to update an aircraft.
#[allow(missing_docs, missing_copy_implementations)]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct UpdateAircraftPayload {
    pub id: String,
    /// The UUID of the model.
    pub vehicle_model_id: Option<String>,
    pub last_vertiport_id: Option<String>,
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
#[allow(missing_docs, missing_copy_implementations)]
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
#[allow(missing_docs, missing_copy_implementations)]
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
