//! Definition of the structs used to represent network assets.
//! This file only exists for development purposes. It is to be replaced
//! by networking calls to the database and to be removed upon
//! completion.
//!
//! Types here are different from the openapi types.
#![allow(missing_docs)]

use chrono::{DateTime, TimeZone, Utc};
use duplicate::duplicate_item;
use lipsum::{lipsum, lipsum_title};
use ordered_float::OrderedFloat;
use rand::Rng;
use serde::{Deserialize, Serialize};
use svc_storage_client_grpc::vertipad::Object as VertipadObject;
use svc_storage_client_grpc::{
    vehicle::Object as VehicleObject, vertiport::Object as VertiportObject,
};
// use svc_storage_client_grpc::vehicle::VehicleType;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

/// A wrapper for `OrderedFloat<f64>` for documentation generation purposes.
#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Copy)]
pub struct OrderedFloat64(pub OrderedFloat<f64>);

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

/// A struct representing the operator.
///
/// An operator is a legal entity that owns assets and is a customer of
/// Arrow Cargo. The operator supplies the assets to the network,
/// expects to receive and operate cargo shipments, and is expected to
/// derive revenue from the operation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Operator {
    pub id: String,
    pub name: String,
    pub country: String,
    pub city: String,
    pub address: String,
    pub postal_code: String,
    pub email: String,
    pub phone: String,
    pub website: String,
    pub description: String,
    pub logo: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Operator {
    /// Generate a random operator.
    pub fn random() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: lipsum_title(),
            country: lipsum(1),
            city: lipsum(2),
            address: lipsum(6),
            postal_code: lipsum(1),
            email: lipsum(1),
            phone: lipsum(1),
            website: lipsum(1),
            description: lipsum(10),
            logo: lipsum(1),
            created_at: Utc::now(),
            updated_at: None,
        }
    }
}

// =====================================================================
// Implementations of helper tools for the assets structs
// =====================================================================

/// A struct representing a group of assets.
///
/// The asset group can be delegated to another operator.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct AssetGroup {
    /// UUID of the asset group.
    pub id: String,
    pub name: Option<String>,
    /// The UUID of an [`crate::structs::Operator`] struct.
    pub owner: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    /// The UUID of an [`crate::structs::Operator`] struct, if available.
    pub delegatee: Option<String>,
    /// The UUIDs of the assets in the group.
    pub assets: Vec<String>,
}

impl AssetGroup {
    /// Generate a random asset group.
    pub fn random() -> Self {
        let num_assets = rand::thread_rng().gen_range(0..=10);
        let mut assets = Vec::with_capacity(num_assets);
        for _ in 0..num_assets {
            assets.push(Uuid::new_v4().to_string());
        }
        Self {
            id: Uuid::new_v4().to_string(),
            name: Some(lipsum_title()),
            owner: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            updated_at: None,
            delegatee: None,
            assets,
        }
    }
}

/// Attributes that are common to all assets.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Basics {
    /// UUID of the asset.
    pub id: String,
    pub name: Option<String>,
    /// The UUID of an [`crate::structs::AssetGroup`] struct, if available.
    pub group_id: Option<String>,
    /// The UUID of an [`crate::structs::Operator`] struct.
    pub owner: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    /// A list of UUIDs of [`crate::structs::Operator`] structs.
    ///
    /// If the vector is empty, the asset is available to everyone.
    ///
    /// Otherwise, the asset is only available to the clients in the
    /// vector.
    pub whitelist: Vec<String>,
    pub status: AssetStatus,
}

/// Get information about an asset.
///
/// This trait is implemented by all asset types.
pub trait AssetsInfo {
    /// Get the asset's basic information.
    fn basics(&self) -> Basics;
    /// Get the asset's ID.
    fn id(&self) -> Result<Uuid, uuid::Error>;
    /// Get the asset's group ID.
    fn group_id(&self) -> Result<Option<Uuid>, uuid::Error>;
    /// Get the asset's name.
    fn name(&self) -> String;
    /// Get the asset's owner.
    fn owner(&self) -> Result<Uuid, uuid::Error>;
    /// Get the asset's creation time.
    fn created_at(&self) -> DateTime<Utc>;
    /// Get the asset's last update time. If the asset has never been
    /// updated, this will return None.
    fn updated_at(&self) -> Option<DateTime<Utc>>;
    /// Check if the asset is grouped.
    fn is_grouped(&self) -> bool;
    /// Check if the asset is open to the public.
    fn is_public(&self) -> bool;
    /// Get the list of clients that have access to the asset.
    fn whitelist(&self) -> Result<Vec<Uuid>, uuid::Error>;
    /// Get the status of the asset.
    fn status(&self) -> AssetStatus;
}

#[duplicate_item(asset; [Aircraft]; [Vertiport])]
impl AssetsInfo for asset {
    fn basics(&self) -> Basics {
        self.basics.clone()
    }
    fn id(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.basics().id)
    }
    fn group_id(&self) -> Result<Option<Uuid>, uuid::Error> {
        match &self.basics().group_id {
            Some(id) => Ok(Some(Uuid::parse_str(id)?)),
            None => Ok(None),
        }
    }
    fn name(&self) -> String {
        self.full_name()
    }
    fn owner(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.basics().owner)
    }
    fn created_at(&self) -> DateTime<Utc> {
        self.basics().created_at
    }
    fn updated_at(&self) -> Option<DateTime<Utc>> {
        self.basics().updated_at
    }
    fn is_grouped(&self) -> bool {
        self.basics().group_id.is_some()
    }
    fn is_public(&self) -> bool {
        self.basics().whitelist.is_empty()
    }
    fn whitelist(&self) -> Result<Vec<Uuid>, uuid::Error> {
        let mut whitelist = Vec::with_capacity(self.basics().whitelist.len());
        for id in &self.basics().whitelist {
            whitelist.push(Uuid::parse_str(id)?);
        }
        Ok(whitelist)
    }
    fn status(&self) -> AssetStatus {
        self.basics().status
    }
}

/// Status of an asset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub enum AssetStatus {
    /// The asset is available for use.
    Available,
    /// The asset is unavailable for use.
    Unavailable,
    /// The asset is only available for emergencies.
    Emergency,
}

/// A struct representing a location.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Location {
    pub latitude: OrderedFloat64,
    pub longitude: OrderedFloat64,
}

// =====================================================================
// Asset types
// =====================================================================

/// A struct representing an aircraft.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Aircraft {
    pub basics: Basics,
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
    pub max_payload_kg: OrderedFloat64,
    pub max_range_km: OrderedFloat64,
    pub last_maintenance: Option<DateTime<Utc>>,
    pub next_maintenance: Option<DateTime<Utc>>,
    pub last_vertiport_id: Option<String>,
}

impl Aircraft {
    /// Get the aircraft's name.
    ///
    /// If the aircraft does not have a name, this will be `manufacturer
    /// model registration_number`.
    ///
    /// For example, "Boeing 737-800 N12345".
    pub fn full_name(&self) -> String {
        match &self.basics.name {
            Some(name) => name.clone(),
            None => format!(
                "{} {} {}",
                self.manufacturer, self.model, self.registration_number
            ),
        }
    }

    #[allow(dead_code)]
    /// Generate a random aircraft.
    pub fn random() -> Self {
        Self {
            basics: Basics {
                id: Uuid::new_v4().to_string(),
                group_id: None,
                name: Some(lipsum_title()),
                owner: Uuid::new_v4().to_string(),
                created_at: Utc::now(),
                updated_at: None,
                whitelist: Vec::new(),
                status: AssetStatus::Available,
            },
            manufacturer: lipsum_title(),
            model: lipsum(12),
            serial_number: lipsum(12),
            registration_number: lipsum(12),
            description: None,
            max_payload_kg: OrderedFloat64::from(1000.0),
            max_range_km: OrderedFloat64::from(1000.0),
            last_maintenance: None,
            next_maintenance: None,
            last_vertiport_id: Some(Uuid::new_v4().to_string()),
        }
    }

    pub fn from(storage_vehicle: VehicleObject) -> Result<Aircraft, String> {
        let data = storage_vehicle.data;
        if data.is_none() {
            return Err("Vehicle data is missing".to_string());
        }
        let data = match data {
            Some(data) => data,
            None => return Err("Vehicle data is missing".to_string()),
        };

        Ok(Aircraft {
            basics: Basics {
                id: storage_vehicle.id,
                group_id: data.asset_group_id,
                name: None,
                owner: Uuid::new_v4().to_string(),
                created_at: Utc::now(),
                updated_at: None,
                whitelist: Vec::new(),
                status: AssetStatus::Available,
            },
            manufacturer: "Arrow".to_string(),
            model: data.vehicle_model_id,
            serial_number: data.serial_number,
            registration_number: data.registration_number,
            description: data.description,
            max_payload_kg: 0.0.into(),
            max_range_km: 0.0.into(),
            last_maintenance: if data.last_maintenance.is_some() {
                Some(
                    Utc.timestamp_opt(data.last_maintenance.unwrap().seconds, 0)
                        .single()
                        .unwrap(),
                )
            } else {
                None
            },
            next_maintenance: if data.next_maintenance.is_some() {
                Some(
                    Utc.timestamp_opt(data.next_maintenance.unwrap().seconds, 0)
                        .single()
                        .unwrap(),
                )
            } else {
                None
            },
            last_vertiport_id: data.last_vertiport_id,
        })
    }
}

/// A struct representing a vertipad (a vertical landing pad).
///
/// A vertipad is a landing pad that is used for vertical takeoff and
/// landing (VTOL) aircraft. Usually, vertipads belong to vertiports.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Vertipad {
    pub id: String,
    pub vertiport_id: String,
    pub enabled: bool,
    pub occupied: bool,
    pub location: Location,
}

impl Vertipad {
    #[allow(dead_code)]
    /// Generate a random vertipad.
    pub fn random() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            vertiport_id: Uuid::new_v4().to_string(),
            enabled: true,
            occupied: false,
            location: Location {
                latitude: OrderedFloat64::from(0.0),
                longitude: OrderedFloat64::from(0.0),
            },
        }
    }

    pub fn from(vertipad: VertipadObject) -> Result<Vertipad, String> {
        let data = vertipad.data;
        if let Some(data) = data {
            Ok(Vertipad {
                id: vertipad.id,
                vertiport_id: data.vertiport_id,
                enabled: data.enabled,
                occupied: data.occupied,
                location: Location {
                    latitude: OrderedFloat64::from(data.latitude),
                    longitude: OrderedFloat64::from(data.longitude),
                },
            })
        } else {
            Err("Vertipad data is missing".to_string())
        }
    }
}

/// A struct representing a vertiport (a vertical airport).
///
/// A vertiport is an airport that is used for vertical takeoff and
/// landing (VTOL) aircraft. A vertiport may have one or more vertipads.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Vertiport {
    pub basics: Basics,
    pub description: Option<String>,
    pub location: Location,
}

impl Vertiport {
    /// Get the vertiport's name.
    ///
    /// It is recommended to make the `name` field required for
    /// vertiports. This can be done through frontend validation.
    pub fn full_name(&self) -> String {
        match &self.basics.name {
            Some(name) => name.clone(),
            None => "Unnamed vertiport".to_string(),
        }
    }

    #[allow(dead_code)]
    /// Generate a random vertiport.
    pub fn random() -> Self {
        Self {
            basics: Basics {
                id: Uuid::new_v4().to_string(),
                group_id: None,
                name: Some(lipsum_title()),
                owner: Uuid::new_v4().to_string(),
                created_at: Utc::now(),
                updated_at: None,
                whitelist: Vec::new(),
                status: AssetStatus::Available,
            },
            description: None,
            location: Location {
                latitude: OrderedFloat64::from(0.0),
                longitude: OrderedFloat64::from(0.0),
            },
        }
    }

    pub fn from(storage_vertiport: VertiportObject) -> Result<Vertiport, String> {
        let data = storage_vertiport.data;
        if let Some(data) = data {
            Ok(Vertiport {
                basics: Basics {
                    id: storage_vertiport.id,
                    group_id: None,
                    name: Some("Vertiport".to_string()),
                    owner: Uuid::new_v4().to_string(),
                    created_at: Utc::now(),
                    updated_at: None,
                    whitelist: Vec::new(),
                    status: AssetStatus::Available,
                },
                description: Some(data.description),
                location: Location {
                    latitude: OrderedFloat64::from(data.latitude),
                    longitude: OrderedFloat64::from(data.longitude),
                },
            })
        } else {
            Err("Vertiport data is missing".to_string())
        }
    }
}

// =====================================================================
// Tests
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_basics() {
        let basics = Basics {
            id: Uuid::new_v4().to_string(),
            name: Some("Test asset".to_string()),
            group_id: Some(Uuid::new_v4().to_string()),
            owner: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
            whitelist: vec![Uuid::new_v4().to_string()],
            status: AssetStatus::Available,
        };
        let asset = Aircraft {
            basics: basics.clone(),
            manufacturer: "Boeing".to_string(),
            model: "737-800".to_string(),
            serial_number: "12345".to_string(),
            registration_number: "N12345".to_string(),
            description: None,
            max_payload_kg: OrderedFloat64::from(1000.0),
            max_range_km: OrderedFloat64::from(1000.0),
            last_maintenance: None,
            next_maintenance: None,
            last_vertiport_id: None,
        };
        assert_eq!(asset.id(), Uuid::parse_str(&basics.id));
        assert_eq!(asset.name(), basics.name.unwrap());
        assert_eq!(asset.owner(), Uuid::parse_str(&basics.owner));
        assert_eq!(asset.created_at(), basics.created_at);
        assert_eq!(asset.updated_at(), basics.updated_at);
        assert_eq!(asset.is_grouped(), true);
        assert_eq!(asset.is_public(), false);
        assert_eq!(asset.status(), basics.status);
    }

    #[test]
    fn test_assets_info_trait_methods_on_different_asset_types() {
        let basics = Basics {
            id: Uuid::new_v4().to_string(),
            name: Some("Test asset".to_string()),
            group_id: Some(Uuid::new_v4().to_string()),
            owner: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
            whitelist: vec![Uuid::new_v4().to_string()],
            status: AssetStatus::Available,
        };
        let aircraft = Aircraft {
            basics: basics.clone(),
            manufacturer: "Boeing".to_string(),
            model: "737-800".to_string(),
            serial_number: "12345".to_string(),
            registration_number: "N12345".to_string(),
            description: None,
            max_payload_kg: OrderedFloat64::from(1000.0),
            max_range_km: OrderedFloat64::from(1000.0),
            last_maintenance: None,
            next_maintenance: None,
            last_vertiport_id: None,
        };

        let vertiport = Vertiport {
            basics: basics.clone(),
            description: None,
            location: Location {
                latitude: 0.0.into(),
                longitude: 0.0.into(),
            },
        };
        assert_eq!(aircraft.id(), Uuid::parse_str(&basics.id));
        assert_eq!(aircraft.name(), basics.name.clone().unwrap());
        assert_eq!(aircraft.owner(), Uuid::parse_str(&basics.owner));
        assert_eq!(aircraft.created_at(), basics.created_at);
        assert_eq!(aircraft.updated_at(), basics.updated_at);
        assert_eq!(aircraft.is_grouped(), true);
        assert_eq!(aircraft.is_public(), false);
        assert_eq!(aircraft.status(), basics.status);

        assert_eq!(vertiport.id(), Uuid::parse_str(&basics.id));
        assert_eq!(vertiport.name(), basics.name.clone().unwrap());
        assert_eq!(vertiport.owner(), Uuid::parse_str(&basics.owner));
        assert_eq!(vertiport.created_at(), basics.created_at);
        assert_eq!(vertiport.updated_at(), basics.updated_at);
        assert_eq!(vertiport.is_grouped(), true);
        assert_eq!(vertiport.is_public(), false);
        assert_eq!(vertiport.status(), basics.status);
    }

    #[test]
    fn test_asset_group() {
        let group_id = Uuid::new_v4().to_string();

        let basics = Basics {
            id: Uuid::new_v4().to_string(),
            name: Some("Test asset".to_string()),
            group_id: Some(group_id.clone()),
            owner: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
            whitelist: vec![Uuid::new_v4().to_string()],
            status: AssetStatus::Available,
        };
        let aircraft = Aircraft {
            basics: basics.clone(),
            manufacturer: "Boeing".to_string(),
            model: "737-800".to_string(),
            serial_number: "12345".to_string(),
            registration_number: "N12345".to_string(),
            description: None,
            max_payload_kg: OrderedFloat64::from(1000.0),
            max_range_km: OrderedFloat64::from(1000.0),
            last_maintenance: None,
            next_maintenance: None,
            last_vertiport_id: None,
        };
        let vertiport = Vertiport {
            basics: basics.clone(),
            description: None,
            location: Location {
                latitude: 0.0.into(),
                longitude: 0.0.into(),
            },
        };

        let asset_group = AssetGroup {
            // pub id: Uuid,
            // pub name: Option<String>,
            // pub owner: Uuid,
            // pub created_at: DateTime<Utc>,
            // pub updated_at: Option<DateTime<Utc>>,
            // pub delegatee: Option<Uuid>,
            // pub assets: Vec<Uuid>,
            id: group_id.clone(),
            name: Some("Test group".to_string()),
            owner: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
            delegatee: None,
            assets: vec![
                aircraft.id().unwrap().to_string(),
                vertiport.id().unwrap().to_string(),
            ],
        };

        assert_eq!(&asset_group.id, &group_id);
        assert_eq!(asset_group.name, Some("Test group".to_string()));

        assert_eq!(asset_group.assets.len(), 2);
        assert_eq!(asset_group.assets[0], aircraft.id().unwrap().to_string());
        assert_eq!(asset_group.assets[1], vertiport.id().unwrap().to_string());

        assert_eq!(
            &aircraft.group_id().unwrap().unwrap().to_string(),
            &group_id
        );
        assert_eq!(
            &vertiport.group_id().unwrap().unwrap().to_string(),
            &group_id
        );
    }
}
