// //! Types used for REST communication with the svc-asset server.

// use serde::{Deserialize, Serialize};
// use std::time::SystemTime;
// use utoipa::{IntoParams, ToSchema};

// use crate::structs::{Aircraft, AssetGroup, Basics, Vertipad, Vertiport};

// //-----------------------------------------------------------
// // Asset groups management (POST, PUT, DELETE)
// //-----------------------------------------------------------

// /// Request to create an asset group.
// #[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, IntoParams)]
// pub struct CreateAssetGroup {
//     pub asset_group: AssetGroup,
// }
