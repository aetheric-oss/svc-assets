//! Definition of the structs used to represent network assets.
//! This file only exists for development purposes. It is to be replaced
//! by networking calls to the database and to be removed upon
//! completion.
//!
//! Types here are different from the openapi types.

use duplicate::duplicate_item;
use lipsum::{lipsum, lipsum_title};
use ordered_float::OrderedFloat;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

/// A struct representing the operator.
///
/// An operator is a legal entity that owns assets and is a customer of
/// Arrow Cargo. The operator supplies the assets to the network,
/// expects to receive and operate cargo shipments, and is expected to
/// derive revenue from the operation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Operator {
    pub id: Uuid,
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
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
}

impl Operator {
    /// Generate a random operator.
    pub fn random() -> Self {
        Self {
            id: Uuid::new_v4(),
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
            created_at: SystemTime::now(),
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetGroup {
    pub id: Uuid,
    pub name: Option<String>,
    pub owner: Uuid,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub delegatee: Option<Uuid>,
    pub assets: Vec<Uuid>,
}

impl AssetGroup {
    /// Generate a random asset group.
    pub fn random() -> Self {
        let num_assets = rand::thread_rng().gen_range(0..=10);
        let mut assets = Vec::with_capacity(num_assets);
        for _ in 0..num_assets {
            assets.push(Uuid::new_v4());
        }
        Self {
            id: Uuid::new_v4(),
            name: Some(lipsum_title()),
            owner: Uuid::new_v4(),
            created_at: SystemTime::now(),
            updated_at: None,
            delegatee: None,
            assets,
        }
    }
}

/// Attributes that are common to all assets.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Basics {
    pub id: Uuid,
    pub name: Option<String>,
    /// The UUID of an [`AssetGroup`] struct, if available.
    pub group_id: Option<Uuid>,
    pub owner: Uuid,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    /// If the vector is empty, the asset is available to everyone.
    ///
    /// Otherwise, the asset is only available to the clients in the
    /// vector.
    pub whitelist: Vec<Uuid>,
    pub status: AssetStatus,
}

/// Get information about an asset.
///
/// This trait is implemented by all asset types.
pub trait AssetsInfo {
    /// Get the asset's basic information.
    fn basics(&self) -> Basics;
    /// Get the asset's ID.
    fn id(&self) -> Uuid;
    /// Get the asset's group ID.
    fn group_id(&self) -> Option<Uuid>;
    /// Get the asset's name.
    fn name(&self) -> String;
    /// Get the asset's owner.
    fn owner(&self) -> Uuid;
    /// Get the asset's creation time.
    fn created_at(&self) -> SystemTime;
    /// Get the asset's last update time. If the asset has never been
    /// updated, this will return None.
    fn updated_at(&self) -> Option<SystemTime>;
    /// Check if the asset is grouped.
    fn is_grouped(&self) -> bool;
    /// Check if the asset is open to the public.
    fn is_public(&self) -> bool;
    /// Get the list of clients that have access to the asset.
    fn whitelist(&self) -> Vec<Uuid>;
    /// Get the status of the asset.
    fn status(&self) -> AssetStatus;
}

#[duplicate_item(asset; [Aircraft]; [Vertiport])]
impl AssetsInfo for asset {
    fn basics(&self) -> Basics {
        self.basics.clone()
    }
    fn id(&self) -> Uuid {
        self.basics().id
    }
    fn group_id(&self) -> Option<Uuid> {
        self.basics().group_id
    }
    fn name(&self) -> String {
        self.full_name()
    }
    fn owner(&self) -> Uuid {
        self.basics().owner
    }
    fn created_at(&self) -> SystemTime {
        self.basics().created_at
    }
    fn updated_at(&self) -> Option<SystemTime> {
        self.basics().updated_at
    }
    fn is_grouped(&self) -> bool {
        self.basics().group_id.is_some()
    }
    fn is_public(&self) -> bool {
        self.basics().whitelist.is_empty()
    }
    fn whitelist(&self) -> Vec<Uuid> {
        self.basics().whitelist
    }
    fn status(&self) -> AssetStatus {
        self.basics().status
    }
}

/// Status of an asset.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetStatus {
    /// The asset is available for use.
    Available,
    /// The asset is unavailable for use.
    Unavailable,
    /// The asset is only available for emergencies.
    Emergency,
}

/// A struct representing a location.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Location {
    pub latitude: OrderedFloat<f64>,
    pub longitude: OrderedFloat<f64>,
}

// =====================================================================
// Asset types
// =====================================================================

/// A struct representing an aircraft.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Aircraft {
    pub basics: Basics,
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

    /// Generate a random aircraft.
    pub fn random() -> Self {
        Self {
            basics: Basics {
                id: Uuid::new_v4(),
                group_id: None,
                name: Some(lipsum_title()),
                owner: Uuid::new_v4(),
                created_at: SystemTime::now(),
                updated_at: None,
                whitelist: Vec::new(),
                status: AssetStatus::Available,
            },
            manufacturer: lipsum_title(),
            model: lipsum(12),
            serial_number: lipsum(12),
            registration_number: lipsum(12),
            description: None,
            max_payload_kg: OrderedFloat(100.0),
            max_range_km: None,
        }
    }
}

/// A struct representing a vertipad (a vertical landing pad).
///
/// A vertipad is a landing pad that is used for vertical takeoff and
/// landing (VTOL) aircraft. Usually, vertipads belong to vertiports.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vertipad {
    pub id: Uuid,
    pub vertiport_id: Uuid,
    pub status: AssetStatus,
    pub location: Location,
}

impl Vertipad {
    /// Generate a random vertipad.
    pub fn random() -> Self {
        Self {
            id: Uuid::new_v4(),
            vertiport_id: Uuid::new_v4(),
            status: AssetStatus::Available,
            location: Location {
                latitude: OrderedFloat(0.0),
                longitude: OrderedFloat(0.0),
            },
        }
    }
}

/// A struct representing a vertiport (a vertical airport).
///
/// A vertiport is an airport that is used for vertical takeoff and
/// landing (VTOL) aircraft. A vertiport may have one or more vertipads.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vertiport {
    pub basics: Basics,
    pub description: Option<String>,
    pub location: Location,
    pub vertipads: Vec<Uuid>,
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

    /// Generate a random vertiport.
    pub fn random() -> Self {
        Self {
            basics: Basics {
                id: Uuid::new_v4(),
                group_id: None,
                name: Some(lipsum_title()),
                owner: Uuid::new_v4(),
                created_at: SystemTime::now(),
                updated_at: None,
                whitelist: Vec::new(),
                status: AssetStatus::Available,
            },
            description: None,
            location: Location {
                latitude: OrderedFloat(0.0),
                longitude: OrderedFloat(0.0),
            },
            vertipads: Vec::new(),
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
            id: Uuid::new_v4(),
            name: Some("Test asset".to_string()),
            group_id: Some(Uuid::new_v4()),
            owner: Uuid::new_v4(),
            created_at: SystemTime::now(),
            updated_at: Some(SystemTime::now()),
            whitelist: vec![Uuid::new_v4()],
            status: AssetStatus::Available,
        };
        let asset = Aircraft {
            basics: basics.clone(),
            manufacturer: "Boeing".to_string(),
            model: "737-800".to_string(),
            serial_number: "12345".to_string(),
            registration_number: "N12345".to_string(),
            description: None,
            max_payload_kg: OrderedFloat(100.0),
            max_range_km: None,
        };
        assert_eq!(asset.id(), basics.id);
        assert_eq!(asset.name(), basics.name.unwrap());
        assert_eq!(asset.owner(), basics.owner);
        assert_eq!(asset.created_at(), basics.created_at);
        assert_eq!(asset.updated_at(), basics.updated_at);
        assert_eq!(asset.is_grouped(), true);
        assert_eq!(asset.is_public(), false);
        assert_eq!(asset.whitelist(), basics.whitelist);
        assert_eq!(asset.status(), basics.status);
    }

    #[test]
    fn test_assets_info_trait_methods_on_different_asset_types() {
        let basics = Basics {
            id: Uuid::new_v4(),
            name: Some("Test asset".to_string()),
            group_id: Some(Uuid::new_v4()),
            owner: Uuid::new_v4(),
            created_at: SystemTime::now(),
            updated_at: Some(SystemTime::now()),
            whitelist: vec![Uuid::new_v4()],
            status: AssetStatus::Available,
        };
        let aircraft = Aircraft {
            basics: basics.clone(),
            manufacturer: "Boeing".to_string(),
            model: "737-800".to_string(),
            serial_number: "12345".to_string(),
            registration_number: "N12345".to_string(),
            description: None,
            max_payload_kg: OrderedFloat(100.0),
            max_range_km: None,
        };

        let vertiport = Vertiport {
            basics: basics.clone(),
            description: None,
            location: Location {
                latitude: 0.0.into(),
                longitude: 0.0.into(),
            },
            vertipads: vec![Uuid::new_v4()],
        };
        assert_eq!(aircraft.id(), basics.id);
        assert_eq!(aircraft.name(), basics.name.clone().unwrap());
        assert_eq!(aircraft.owner(), basics.owner);
        assert_eq!(aircraft.created_at(), basics.created_at);
        assert_eq!(aircraft.updated_at(), basics.updated_at);
        assert_eq!(aircraft.is_grouped(), true);
        assert_eq!(aircraft.is_public(), false);
        assert_eq!(aircraft.whitelist(), basics.whitelist);
        assert_eq!(aircraft.status(), basics.status);

        assert_eq!(vertiport.id(), basics.id);
        assert_eq!(vertiport.name(), basics.name.clone().unwrap());
        assert_eq!(vertiport.owner(), basics.owner);
        assert_eq!(vertiport.created_at(), basics.created_at);
        assert_eq!(vertiport.updated_at(), basics.updated_at);
        assert_eq!(vertiport.is_grouped(), true);
        assert_eq!(vertiport.is_public(), false);
        assert_eq!(vertiport.whitelist(), basics.whitelist);
        assert_eq!(vertiport.status(), basics.status);
    }

    #[test]
    fn test_asset_group() {
        let group_id = Uuid::new_v4();

        let basics = Basics {
            id: Uuid::new_v4(),
            name: Some("Test asset".to_string()),
            group_id: Some(group_id.clone()),
            owner: Uuid::new_v4(),
            created_at: SystemTime::now(),
            updated_at: Some(SystemTime::now()),
            whitelist: vec![Uuid::new_v4()],
            status: AssetStatus::Available,
        };
        let aircraft = Aircraft {
            basics: basics.clone(),
            manufacturer: "Boeing".to_string(),
            model: "737-800".to_string(),
            serial_number: "12345".to_string(),
            registration_number: "N12345".to_string(),
            description: None,
            max_payload_kg: OrderedFloat(100.0),
            max_range_km: None,
        };
        let vertiport = Vertiport {
            basics: basics.clone(),
            description: None,
            location: Location {
                latitude: 0.0.into(),
                longitude: 0.0.into(),
            },
            vertipads: vec![Uuid::new_v4()],
        };

        let asset_group = AssetGroup {
            // pub id: Uuid,
            // pub name: Option<String>,
            // pub owner: Uuid,
            // pub created_at: SystemTime,
            // pub updated_at: Option<SystemTime>,
            // pub delegatee: Option<Uuid>,
            // pub assets: Vec<Uuid>,
            id: group_id,
            name: Some("Test group".to_string()),
            owner: Uuid::new_v4(),
            created_at: SystemTime::now(),
            updated_at: Some(SystemTime::now()),
            delegatee: None,
            assets: vec![aircraft.id(), vertiport.id()],
        };

        assert_eq!(asset_group.id, group_id);
        assert_eq!(asset_group.name, Some("Test group".to_string()));

        assert_eq!(asset_group.assets.len(), 2);
        assert_eq!(asset_group.assets[0], aircraft.id());
        assert_eq!(asset_group.assets[1], vertiport.id());

        assert_eq!(aircraft.group_id(), Some(group_id));
        assert_eq!(vertiport.group_id(), Some(group_id));
    }
}
