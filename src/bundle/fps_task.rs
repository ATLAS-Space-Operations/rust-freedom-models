use std::collections::HashMap;

use time::OffsetDateTime;

use crate::{account::AccountCidr, band::BandType, task::TaskType};

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct SiteHardware {
    pub manufacturer: String,
    pub model: String,
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Default, Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Band {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub typ: Option<BandType>,
    pub default_band_width_mghz: f64,
    pub id: i32,
}

/// A bundle representing all of the information required by the FPS and Gateway in order to execute
/// a pass.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Bundle {
    pub account_name: String,
    pub account_id: i32,
    pub account_storage_key: String,
    pub account_storage_group: Option<String>,
    pub account_cidr: Vec<AccountCidr>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub account_internal_metadata: HashMap<String, String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bands: Vec<Band>,
    pub satellite_name: String,
    pub satellite_id: i32,
    pub site_name: String,
    pub site_id: i32,
    pub site_configuration_name: String,
    pub site_configuration_id: i32,
    pub site_configuration_hardware: Vec<SiteHardware>,
    pub task_id: i32,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub task_start: OffsetDateTime,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub task_end: OffsetDateTime,
    pub task_request_type: TaskType,
    pub task_request_test_file: String,
    pub task_request_id: i32,
}

impl Default for Bundle {
    fn default() -> Self {
        Self {
            account_name: Default::default(),
            account_id: Default::default(),
            account_storage_key: Default::default(),
            account_storage_group: Default::default(),
            account_cidr: Default::default(),
            account_internal_metadata: Default::default(),
            bands: Default::default(),
            satellite_name: Default::default(),
            satellite_id: Default::default(),
            site_name: Default::default(),
            site_id: Default::default(),
            site_configuration_name: Default::default(),
            site_configuration_id: Default::default(),
            site_configuration_hardware: Default::default(),
            task_id: Default::default(),
            task_start: OffsetDateTime::UNIX_EPOCH,
            task_end: OffsetDateTime::UNIX_EPOCH + std::time::Duration::from_secs(60),
            task_request_type: TaskType::Exact,
            task_request_test_file: Default::default(),
            task_request_id: Default::default(),
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    #[test]
    fn deserialize_bundle() {
        let file = std::fs::read_to_string("resources/fps_bundle/1.json").unwrap();
        let _val: Vec<Bundle> = serde_json::from_str(&file).unwrap();
    }
}
