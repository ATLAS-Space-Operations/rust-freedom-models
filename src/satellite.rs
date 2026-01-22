#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use url::Url;

use crate::Hateoas;

#[cfg(feature = "serde")]
use super::utils;

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct TwoLineElement {
    pub line1: String,
    pub line2: String,
}

/// The Freedom API representation of a satellite.
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Satellite {
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub created: OffsetDateTime,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub modified: Option<OffsetDateTime>,
    pub name: String,
    pub description: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub norad_cat_id: Option<u32>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub tle: Option<TwoLineElement>,
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

impl Hateoas for Satellite {
    fn get_links(&self) -> &HashMap<String, url::Url> {
        &self.links
    }

    fn get_links_mut(&mut self) -> &mut HashMap<String, url::Url> {
        &mut self.links
    }
}
