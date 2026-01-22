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
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct UserPreferences {
    pub visibility_days: u32,
    pub min_elevation: f32,
    pub max_elevation: f32,
    pub min_duration: f32,
    pub elevation_tolerance: f32,
    pub duration_tolerance: f32,
    pub notify_via_email: bool,
    pub notify_via_text: bool,
}

/// A user in freedom
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct User {
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub created: OffsetDateTime,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub modified: Option<OffsetDateTime>,
    pub first_name: String,
    pub last_name: String,
    pub verified: bool,
    pub email: String,
    pub preferences: UserPreferences,
    /// Unavailable for user accounts
    #[cfg_attr(feature = "serde", serde(default))]
    pub internal_meta_data: Option<HashMap<String, String>>,
    /// Unavailable for user accounts
    #[cfg_attr(feature = "serde", serde(default))]
    pub deleted: Option<bool>,
    pub api_access_enabled: bool,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "_links", with = "utils::links::serde", default)
    )]
    pub links: HashMap<String, Url>,
}

impl Hateoas for User {
    fn get_links(&self) -> &HashMap<String, url::Url> {
        &self.links
    }

    fn get_links_mut(&mut self) -> &mut HashMap<String, url::Url> {
        &mut self.links
    }
}
