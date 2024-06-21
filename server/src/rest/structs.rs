//! Definition of the structs used to represent network assets.
//! This file only exists for development purposes. It is to be replaced
//! by networking calls to the database and to be removed upon
//! completion.
//!
//! Types here are different from the openapi types.
#![allow(missing_docs)]

use crate::rest::api::{aircraft::Aircraft, vertipad::Vertipad, vertiport::Vertiport};
use duplicate::duplicate_item;
use lib_common::time::{DateTime, Utc};
use lib_common::uuid::{to_uuid, Uuid};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use utoipa::{IntoParams, ToSchema};

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

/// Attributes that are common to all assets.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Basics {
    /// UUID of the asset.
    pub id: String,
    /// The name of the asset
    pub name: Option<String>,
    /// The UUID of an [`AssetGroup`] struct, if available.
    pub group_id: Option<String>,
    /// Created at time.
    pub created_at: DateTime<Utc>,
    /// Updated at time.
    pub updated_at: DateTime<Utc>,
    /// A list of UUIDs of [`Operator`] structs.
    ///
    /// If the vector is empty, the asset is available to everyone.
    ///
    /// Otherwise, the asset is only available to the clients in the
    /// vector.
    pub whitelist: Vec<String>,

    /// The status of the asset
    pub status: AssetStatus,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AssetsError {
    InvalidUuid,
}

impl Display for AssetsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AssetsError::InvalidUuid => write!(f, "Invalid UUID"),
        }
    }
}

/// Get information about an asset.
///
/// This trait is implemented by all asset types.
pub trait AssetsInfo {
    /// Get the asset's basic information.
    fn basics(&self) -> Basics;
    /// Get the asset's ID.
    fn id(&self) -> Result<Uuid, AssetsError>;
    /// Get the asset's group ID.
    fn group_id(&self) -> Result<Option<Uuid>, AssetsError>;
    /// Get the asset's name.
    fn name(&self) -> String;
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
    fn whitelist(&self) -> Vec<Uuid>;
    /// Get the status of the asset.
    fn status(&self) -> AssetStatus;
}

#[duplicate_item(asset; [Aircraft]; [Vertiport]; [Vertipad])]
impl AssetsInfo for asset {
    fn basics(&self) -> Basics {
        self.basics.clone()
    }

    fn id(&self) -> Result<Uuid, AssetsError> {
        to_uuid(&self.basics().id).ok_or_else(|| {
            rest_error!("Invalid asset UUID: {}", self.basics().id);
            AssetsError::InvalidUuid
        })
    }

    fn group_id(&self) -> Result<Option<Uuid>, AssetsError> {
        let Some(id) = &self.basics().group_id else {
            return Ok(None);
        };

        let id = to_uuid(id).ok_or_else(|| {
            rest_error!("Invalid group UUID: {}", id);
            AssetsError::InvalidUuid
        })?;

        Ok(Some(id))
    }

    fn name(&self) -> String {
        self.full_name()
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

    fn whitelist(&self) -> Vec<Uuid> {
        self.basics()
            .whitelist
            .iter()
            .filter_map(|id| to_uuid(id))
            .collect()
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

#[cfg(test)]
mod tests {
    use super::*;
    use svc_storage_client_grpc::prelude::*;

    #[test]
    fn test_asset_basics() {
        let whitelisted_id = Uuid::new_v4().to_string();
        let mut basics = Basics {
            id: Uuid::new_v4().to_string(),
            name: Some("Test asset".to_string()),
            group_id: Some(Uuid::new_v4().to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            whitelist: vec![whitelisted_id.clone(), "invalid".to_string()],
            status: AssetStatus::Available,
        };
        let mut asset = Aircraft {
            basics: basics.clone(),
            manufacturer: "Airbus".to_string(),
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

        assert_eq!(asset.id().unwrap(), Uuid::parse_str(&basics.id).unwrap());
        assert_eq!(asset.name(), basics.name.clone().unwrap());
        assert_eq!(asset.created_at(), basics.created_at);
        assert_eq!(asset.updated_at(), basics.updated_at);
        assert_eq!(asset.is_grouped(), true);
        assert_eq!(asset.is_public(), false);
        assert_eq!(asset.status(), basics.status);
        assert_eq!(asset.whitelist().len(), 1); // only one id passes validation
        assert_eq!(asset.whitelist().pop().unwrap().to_string(), whitelisted_id);

        // invalid id
        basics.id = "invalid".to_string();
        asset.basics = basics.clone();
        assert_eq!(asset.id(), Err(AssetsError::InvalidUuid));

        // invalid group id
        basics.id = Uuid::new_v4().to_string();
        basics.group_id = Some("invalid".to_string());
        asset.basics = basics.clone();
        assert_eq!(asset.group_id(), Err(AssetsError::InvalidUuid));
    }

    #[test]
    fn test_asset_group() {
        let group_id = Uuid::new_v4().to_string();

        let basics = Basics {
            id: Uuid::new_v4().to_string(),
            name: Some("Test asset".to_string()),
            group_id: Some(group_id.clone()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            whitelist: vec![Uuid::new_v4().to_string()],
            status: AssetStatus::Available,
        };
        let aircraft = vehicle::mock::get_data_obj();
        let aircraft = Aircraft {
            basics: basics.clone(),
            manufacturer: "Airbus".to_string(),
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
