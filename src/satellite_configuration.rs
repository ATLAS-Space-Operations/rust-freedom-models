#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use url::Url;

use crate::Hateoas;

#[cfg(feature = "serde")]
use super::utils;

/// The Freedom API representation of a satellite.
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct SatelliteConfiguration {
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub created: OffsetDateTime,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub modified: Option<OffsetDateTime>,
    pub name: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub orbit: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub notes: String,
    #[cfg_attr(feature = "serde", serde(rename = "pullTLE"))]
    pub pull_tle: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub internal_meta_data: Option<HashMap<String, String>>,
    pub account_name: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub meta_data: Option<HashMap<String, String>>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "_links", with = "utils::links::serde", default)
    )]
    pub links: HashMap<String, Url>,
}

impl Hateoas for SatelliteConfiguration {
    fn get_links(&self) -> &HashMap<String, url::Url> {
        &self.links
    }

    fn get_links_mut(&mut self) -> &mut HashMap<String, url::Url> {
        &mut self.links
    }
}
