//! Definition of the structs used to represent network assets.
//! This file only exists for development purposes. It is to be replaced
//! by networking calls to the database and to be removed upon
//! completion.
//!
//! Types here are different from the openapi types.
#![allow(missing_docs)]

use chrono::{DateTime, Utc};
use duplicate::duplicate_item;
use lipsum::{lipsum, lipsum_title};
use rand::Rng;
use serde::{Deserialize, Serialize};
use svc_storage_client_grpc::prelude::vertipad::Object as VertipadObject;
use svc_storage_client_grpc::prelude::{
    vehicle::Object as VehicleObject, vertiport::Object as VertiportObject,
};
use svc_storage_client_grpc::prelude::{GeoPoint, GeoPolygon};
// use svc_storage_client_grpc::vehicle::VehicleType;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

/// A struct representing the operator.
///
/// An operator is a legal entity that owns assets and is a customer of
/// Arrow Cargo. The operator supplies the assets to the network,
/// expects to receive and operate cargo shipments, and is expected to
/// derive revenue from the operation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, IntoParams)]
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
    pub updated_at: DateTime<Utc>,
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
            updated_at: Utc::now(),
        }
    }
}

// =====================================================================
// Implementations of helper tools for the assets structs
// =====================================================================

/// A struct representing a group of assets.
///
/// The asset group can be delegated to another operator.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct AssetGroup {
    /// UUID of the asset group.
    pub id: String,
    pub name: Option<String>,
    /// The UUID of an [`Operator`] struct.
    pub owner: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    /// The UUID of an [`Operator`] struct, if available.
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
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            delegatee: None,
            assets,
        }
    }
}

/// Attributes that are common to all assets.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Basics {
    /// UUID of the asset.
    pub id: String,
    pub name: Option<String>,
    /// The UUID of an [`AssetGroup`] struct, if available.
    pub group_id: Option<String>,
    /// The UUID of an [`Operator`] struct.
    pub owner: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// A list of UUIDs of [`Operator`] structs.
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
    fn updated_at(&self) -> DateTime<Utc>;
    /// Check if the asset is grouped.
    fn is_grouped(&self) -> bool;
    /// Check if the asset is open to the public.
    fn is_public(&self) -> bool;
    /// Get the list of clients that have access to the asset.
    fn whitelist(&self) -> Result<Vec<Uuid>, uuid::Error>;
    /// Get the status of the asset.
    fn status(&self) -> AssetStatus;
}

#[duplicate_item(asset; [Aircraft]; [Vertiport]; [Vertipad])]
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
    fn updated_at(&self) -> DateTime<Utc> {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum AssetStatus {
    /// The asset is available for use.
    Available,
    /// The asset is unavailable for use.
    Unavailable,
    /// The asset is only available for emergencies.
    Emergency,
}

// =====================================================================
// Asset types
// =====================================================================

/// A struct representing an aircraft.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Aircraft {
    pub basics: Basics,
    /// The aircraft's manufacturer.
    ///
    /// TODO(R4): For now we can just say "Boeing", "Airbus", etc. Later, we
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
    pub last_maintenance: Option<DateTime<Utc>>,
    pub next_maintenance: Option<DateTime<Utc>>,
    pub hangar_id: Option<String>,
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
                id: Uuid::new_v4().to_string(),
                group_id: None,
                name: Some(lipsum_title()),
                owner: Uuid::new_v4().to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                whitelist: Vec::new(),
                status: AssetStatus::Available,
            },
            manufacturer: lipsum_title(),
            model: lipsum(12),
            serial_number: lipsum(12),
            registration_number: lipsum(12),
            description: None,
            max_payload_kg: 1000.0,
            max_range_km: 1000.0,
            last_maintenance: None,
            next_maintenance: None,
            hangar_id: Some(Uuid::new_v4().to_string()),
        }
    }
}

impl TryFrom<VehicleObject> for Aircraft {
    type Error = String;

    fn try_from(object: VehicleObject) -> Result<Self, Self::Error> {
        let data = match object.data {
            Some(data) => data,
            None => return Err("(try_from) vehicle data is missing".to_string()),
        };

        Ok(Aircraft {
            basics: Basics {
                id: object.id,
                group_id: data.asset_group_id,
                name: None,
                owner: Uuid::new_v4().to_string(),
                created_at: data
                    .created_at
                    .ok_or("(try_from) created_at field empty but expected.")?
                    .into(),
                updated_at: data
                    .updated_at
                    .ok_or("(try_from) updated_at field empty but expected.")?
                    .into(),
                whitelist: Vec::new(),
                status: AssetStatus::Available,
            },
            manufacturer: "Arrow".to_string(),
            model: data.vehicle_model_id,
            serial_number: data.serial_number,
            registration_number: data.registration_number,
            description: data.description,
            max_payload_kg: 0.0,
            max_range_km: 0.0,
            last_maintenance: data.last_maintenance.map(|date| date.into()),
            next_maintenance: data.next_maintenance.map(|date| date.into()),
            hangar_id: data.hangar_id,
        })
    }
}

/// A struct representing a vertipad (a vertical landing pad).
///
/// A vertipad is a landing pad that is used for vertical takeoff and
/// landing (VTOL) aircraft. Usually, vertipads belong to vertiports.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Vertipad {
    pub basics: Basics,
    pub vertiport_id: String,
    pub enabled: bool,
    pub occupied: bool,
    pub geo_location: GeoPoint,
    pub schedule: Option<String>,
}

impl Vertipad {
    /// Get the vertipad's name.
    ///
    /// It is recommended to make the `name` field required for
    /// vertiports. This can be done through frontend validation.
    pub fn full_name(&self) -> String {
        match &self.basics.name {
            Some(name) => name.clone(),
            None => "Unnamed vertipad".to_string(),
        }
    }
}

impl TryFrom<VertipadObject> for Vertipad {
    type Error = String;

    fn try_from(object: VertipadObject) -> Result<Self, Self::Error> {
        let data = match object.data {
            Some(data) => data,
            None => return Err("(try_from) vertipad data is missing".to_string()),
        };

        Ok(Vertipad {
            basics: Basics {
                id: object.id,
                group_id: None,
                name: Some(data.name),
                owner: Uuid::new_v4().to_string(),
                created_at: data
                    .created_at
                    .ok_or("(try_from) created_at field empty but expected.")?
                    .into(),
                updated_at: data
                    .updated_at
                    .ok_or("(try_from) updated_at field empty but expected.")?
                    .into(),
                whitelist: Vec::new(),
                status: AssetStatus::Available,
            },
            geo_location: data
                .geo_location
                .ok_or("(try_from) geo_location empty but expected.")?,
            vertiport_id: data.vertiport_id,
            enabled: data.enabled,
            occupied: data.occupied,
            schedule: data.schedule,
        })
    }
}

/// A struct representing a vertiport (a vertical airport).
///
/// A vertiport is an airport that is used for vertical takeoff and
/// landing (VTOL) aircraft. A vertiport may have one or more vertipads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Vertiport {
    pub basics: Basics,
    pub description: String,
    pub geo_location: GeoPolygon,
    pub schedule: Option<String>,
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
}

impl TryFrom<VertiportObject> for Vertiport {
    type Error = String;

    fn try_from(object: VertiportObject) -> Result<Self, Self::Error> {
        let data = match object.data {
            Some(data) => data,
            None => return Err("(try_from) vertiport data is missing".to_string()),
        };

        Ok(Vertiport {
            basics: Basics {
                id: object.id,
                group_id: None,
                name: Some(data.name),
                owner: Uuid::new_v4().to_string(),
                created_at: data
                    .created_at
                    .ok_or("(try_from) created_at field empty but expected.")?
                    .into(),
                updated_at: data
                    .updated_at
                    .ok_or("(try_from) updated_at field empty but expected.")?
                    .into(),
                whitelist: Vec::new(),
                status: AssetStatus::Available,
            },
            description: data.description,
            geo_location: data
                .geo_location
                .ok_or("(try_from) geo_location empty but expected.")?,
            schedule: data.schedule,
        })
    }
}

// =====================================================================
// Tests
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use svc_storage_client_grpc::prelude::*;

    #[test]
    fn test_asset_basics() {
        let basics = Basics {
            id: Uuid::new_v4().to_string(),
            name: Some("Test asset".to_string()),
            group_id: Some(Uuid::new_v4().to_string()),
            owner: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
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
            max_payload_kg: 1000.0,
            max_range_km: 1000.0,
            last_maintenance: None,
            next_maintenance: None,
            hangar_id: None,
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
    fn test_aircraft_basics_trait_methods() {
        let aircraft = vehicle::mock::get_data_obj();
        let basics = Basics {
            id: Uuid::new_v4().to_string(),
            name: Some("Test asset".to_string()),
            group_id: Some(Uuid::new_v4().to_string()),
            owner: Uuid::new_v4().to_string(),
            created_at: aircraft
                .created_at
                .expect("mock didn't return a created_at date.")
                .into(),
            updated_at: aircraft
                .updated_at
                .expect("mock didn't return a updated_at date.")
                .into(),
            whitelist: vec![Uuid::new_v4().to_string()],
            status: AssetStatus::Available,
        };
        let aircraft = Aircraft {
            basics: basics.clone(),
            manufacturer: "Boeing".to_string(),
            model: "737-800".to_string(),
            serial_number: aircraft.serial_number,
            registration_number: aircraft.registration_number,
            description: aircraft.description,
            max_payload_kg: 1000.0,
            max_range_km: 1000.0,
            last_maintenance: None,
            next_maintenance: None,
            hangar_id: None,
        };
        assert_eq!(aircraft.id(), Uuid::parse_str(&basics.id));
        assert_eq!(aircraft.name(), basics.name.clone().unwrap());
        assert_eq!(aircraft.owner(), Uuid::parse_str(&basics.owner));
        assert_eq!(aircraft.created_at(), basics.created_at);
        assert_eq!(aircraft.updated_at(), basics.updated_at);
        assert_eq!(aircraft.is_grouped(), true);
        assert_eq!(aircraft.is_public(), false);
        assert_eq!(aircraft.status(), basics.status);
    }

    #[test]
    fn test_aircraft_from_vehicle_object() {
        let expected_vehicle_id = Uuid::new_v4();
        let vehicle_data: vehicle::Data = vehicle::mock::get_data_obj();
        let vehicle_obj = VehicleObject {
            id: expected_vehicle_id.clone().to_string(),
            data: Some(vehicle_data.clone()),
        };
        let aircraft_result = vehicle_obj.try_into();
        assert!(aircraft_result.is_ok());
        let aircraft: Aircraft = aircraft_result.unwrap();

        assert_eq!(aircraft.id(), Ok(expected_vehicle_id));
        assert!(aircraft.name().len() > 0);
        assert!(aircraft.owner().is_ok());
        assert_eq!(
            aircraft.created_at(),
            vehicle_data
                .created_at
                .expect("mock didn't return a created_at date.")
                .into()
        );
        assert_eq!(
            aircraft.updated_at(),
            vehicle_data
                .updated_at
                .expect("mock didn't return a updated_at date.")
                .into()
        );
        assert_eq!(aircraft.is_grouped(), false);
        assert_eq!(aircraft.is_public(), true);
        assert_eq!(aircraft.status(), AssetStatus::Available);
    }

    #[test]
    fn test_vertiport_basics_trait_methods() {
        let vertiport = vertiport::mock::get_data_obj();
        let basics = Basics {
            id: Uuid::new_v4().to_string(),
            name: Some("Test asset".to_string()),
            group_id: Some(Uuid::new_v4().to_string()),
            owner: Uuid::new_v4().to_string(),
            created_at: vertiport
                .created_at
                .expect("mock didn't return a created_at date.")
                .into(),
            updated_at: vertiport
                .updated_at
                .expect("mock didn't return a updated_at date.")
                .into(),
            whitelist: vec![Uuid::new_v4().to_string()],
            status: AssetStatus::Available,
        };

        let vertiport = Vertiport {
            basics: basics.clone(),
            description: vertiport.description,
            geo_location: vertiport.geo_location.unwrap(),
            schedule: vertiport.schedule,
        };
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
    fn test_vertiport_from_vertiport_object() {
        let expected_vertiport_id = Uuid::new_v4();
        let vertiport_data: vertiport::Data = vertiport::mock::get_data_obj();
        let vertiport_obj = VertiportObject {
            id: expected_vertiport_id.clone().to_string(),
            data: Some(vertiport_data.clone()),
        };
        let vertiport_result = vertiport_obj.try_into();
        assert!(vertiport_result.is_ok());
        let vertiport: Vertiport = vertiport_result.unwrap();

        assert_eq!(vertiport.id(), Ok(expected_vertiport_id));
        assert_eq!(vertiport.name(), vertiport_data.name);
        assert!(vertiport.owner().is_ok());
        assert_eq!(
            vertiport.created_at(),
            vertiport_data
                .created_at
                .expect("mock didn't return a created_at date.")
                .into()
        );
        assert_eq!(
            vertiport.updated_at(),
            vertiport_data
                .updated_at
                .expect("mock didn't return a updated_at date.")
                .into()
        );
        assert_eq!(vertiport.is_grouped(), false);
        assert_eq!(vertiport.is_public(), true);
        assert_eq!(vertiport.status(), AssetStatus::Available);
    }

    #[test]
    fn test_vertipad_basics_trait_methods() {
        let vertipad = vertipad::mock::get_data_obj();
        let basics = Basics {
            id: Uuid::new_v4().to_string(),
            name: Some("Test asset".to_string()),
            group_id: Some(Uuid::new_v4().to_string()),
            owner: Uuid::new_v4().to_string(),
            created_at: vertipad
                .created_at
                .expect("mock didn't return a created_at date.")
                .into(),
            updated_at: vertipad
                .updated_at
                .expect("mock didn't return a updated_at date.")
                .into(),
            whitelist: vec![Uuid::new_v4().to_string()],
            status: AssetStatus::Available,
        };

        let vertipad = Vertipad {
            basics: basics.clone(),
            geo_location: vertipad.geo_location.unwrap(),
            vertiport_id: vertipad.vertiport_id,
            enabled: vertipad.enabled,
            occupied: vertipad.occupied,
            schedule: vertipad.schedule,
        };
        assert_eq!(vertipad.id(), Uuid::parse_str(&basics.id));
        assert_eq!(vertipad.name(), basics.name.clone().unwrap());
        assert_eq!(vertipad.owner(), Uuid::parse_str(&basics.owner));
        assert_eq!(vertipad.created_at(), basics.created_at);
        assert_eq!(vertipad.updated_at(), basics.updated_at);
        assert_eq!(vertipad.is_grouped(), true);
        assert_eq!(vertipad.is_public(), false);
        assert_eq!(vertipad.status(), basics.status);
    }

    #[test]
    fn test_vertipad_from_vertipad_object() {
        let expected_vertipad_id = Uuid::new_v4();
        let vertipad_data: vertipad::Data = vertipad::mock::get_data_obj();
        let vertipad_obj = VertipadObject {
            id: expected_vertipad_id.clone().to_string(),
            data: Some(vertipad_data.clone()),
        };
        let vertipad_result = vertipad_obj.try_into();
        assert!(vertipad_result.is_ok());
        let vertipad: Vertipad = vertipad_result.unwrap();

        assert_eq!(vertipad.id(), Ok(expected_vertipad_id));
        assert_eq!(vertipad.name(), vertipad_data.name);
        assert!(vertipad.owner().is_ok());
        assert_eq!(
            vertipad.created_at(),
            vertipad_data
                .created_at
                .expect("mock didn't return a created_at date.")
                .into()
        );
        assert_eq!(
            vertipad.updated_at(),
            vertipad_data
                .updated_at
                .expect("mock didn't return a updated_at date.")
                .into()
        );
        assert_eq!(vertipad.is_grouped(), false);
        assert_eq!(vertipad.is_public(), true);
        assert_eq!(vertipad.status(), AssetStatus::Available);
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
            updated_at: Utc::now(),
            whitelist: vec![Uuid::new_v4().to_string()],
            status: AssetStatus::Available,
        };
        let aircraft = vehicle::mock::get_data_obj();
        let aircraft = Aircraft {
            basics: basics.clone(),
            manufacturer: "Boeing".to_string(),
            model: "737-800".to_string(),
            serial_number: aircraft.serial_number,
            registration_number: aircraft.registration_number,
            description: aircraft.description,
            max_payload_kg: 1000.0,
            max_range_km: 1000.0,
            last_maintenance: None,
            next_maintenance: None,
            hangar_id: None,
        };

        let vertiport = vertiport::mock::get_data_obj();
        let vertiport = Vertiport {
            basics: basics.clone(),
            description: vertiport.description,
            geo_location: vertiport.geo_location.unwrap(),
            schedule: vertiport.schedule,
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
            created_at: Some(Utc::now()),
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
