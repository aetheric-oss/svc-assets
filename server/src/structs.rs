//! Definition of the structs used to represent network assets.
//! This file only exists for development purposes. It is to be replaced
//! by networking calls to the database and to be removed upon
//! completion.
//!
//! Types here are different from the openapi types.

use duplicate::duplicate_item;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

/// A struct representing the operator.
///
/// An operator is a legal entity that owns assets and is a customer of
/// Arrow Cargo. The operator supplies the assets to the network,
/// expects to receive and operate cargo shipments, and is expected to
/// derive revenue from the operation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    pub created_at: String,
    pub updated_at: String,
}

// =====================================================================
// Implementations of helper tools for the assets structs
// =====================================================================

/// Get information about an asset.
///
/// This trait is implemented by all asset types.
pub trait AssetsInfo {
    /// Get the asset's ID.
    fn id(&self) -> Uuid;
    /// Get the asset's group ID.
    fn group_id(&self) -> Option<Uuid>;
    /// Get the asset's name.
    fn name(&self) -> String;
    /// Get the asset's owner.
    fn owner(&self) -> Uuid;
    /// Get the asset's delegatee. If the asset is not delegated, this
    /// will return None.
    fn delegatee(&self) -> Option<Uuid>;
    /// Get the asset's creation time.
    fn created_at(&self) -> SystemTime;
    /// Get the asset's last update time. If the asset has never been
    /// updated, this will return None.
    fn updated_at(&self) -> Option<SystemTime>;
    /// Check if the asset is grouped.
    fn is_grouped(&self) -> bool;
    /// Check if the asset is delegated.
    fn is_delegated(&self) -> bool;
    /// Check if the asset is open to the public.
    fn is_public(&self) -> bool;
    /// Get the list of clients that have access to the asset.
    fn clients(&self) -> Vec<Uuid>;
    /// Get the status of the asset.
    fn status(&self) -> AssetStatus;
}

#[duplicate_item(asset; [Aircraft]; [Vertiport])]
impl AssetsInfo for asset {
    fn id(&self) -> Uuid {
        self.id
    }
    fn group_id(&self) -> Option<Uuid> {
        self.group_id
    }
    fn name(&self) -> String {
        self.full_name()
    }
    fn owner(&self) -> Uuid {
        self.owner
    }
    fn delegatee(&self) -> Option<Uuid> {
        self.delegatee
    }
    fn created_at(&self) -> SystemTime {
        self.created_at
    }
    fn updated_at(&self) -> Option<SystemTime> {
        self.updated_at
    }
    fn is_grouped(&self) -> bool {
        self.group_id.is_some()
    }
    fn is_delegated(&self) -> bool {
        self.delegatee.is_some()
    }
    fn is_public(&self) -> bool {
        self.restricted_to.is_empty()
    }
    fn clients(&self) -> Vec<Uuid> {
        self.restricted_to.clone()
    }
    fn status(&self) -> AssetStatus {
        self.status.clone()
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
    pub id: Uuid,
    pub name: Option<String>,
    pub group_id: Option<Uuid>,
    pub owner: Uuid,
    pub delegatee: Option<Uuid>,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    /// If the vector is empty, the aircraft is available to everyone.
    /// Otherwise, the aircraft is only available to the clients in the
    /// vector.
    pub restricted_to: Vec<Uuid>,
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
    pub max_payload: OrderedFloat<f32>,
    pub max_range: Option<OrderedFloat<f64>>,
}

impl Aircraft {
    /// Get the aircraft's name.
    ///
    /// If the aircraft does not have a name, this will be `manufacturer
    /// model registration_number`.
    ///
    /// For example, "Boeing 737-800 N12345".
    pub fn full_name(&self) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => format!(
                "{} {} {}",
                self.manufacturer, self.model, self.registration_number
            ),
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

/// A struct representing a vertiport (a vertical airport).
///
/// A vertiport is an airport that is used for vertical takeoff and
/// landing (VTOL) aircraft. A vertiport may have one or more vertipads.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vertiport {
    pub id: Uuid,
    pub name: String,
    pub group_id: Option<Uuid>,
    pub owner: Uuid,
    pub delegatee: Option<Uuid>,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    /// If the vector is empty, the vertiport is available to everyone.
    /// Otherwise, the vertiport is only available to the clients in the
    /// vector.
    pub restricted_to: Vec<Uuid>,
    pub status: AssetStatus,
    pub description: Option<String>,
    pub location: Location,
    pub vertipads: Vec<Uuid>,
}

impl Vertiport {
    /// Get the vertiport's name.
    ///
    /// This is the same as the `name` field, which is a required field.
    pub fn full_name(&self) -> String {
        self.name.clone()
    }
}
