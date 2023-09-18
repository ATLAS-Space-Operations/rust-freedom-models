#[cfg(feature = "hash")]
use derivative::Derivative;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use url::Url;

use crate::azel::Location;
use crate::Hateoas;

#[cfg(feature = "serde")]
use super::utils;

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "hash", derive(Derivative), derivative(Hash))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteHardware {
    pub manual: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ip: Option<String>,
    pub name: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub specifications: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub manufacturer: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub model: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub created: OffsetDateTime,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub modified: Option<OffsetDateTime>,
    #[cfg_attr(feature = "serde", serde(rename = "type", default))]
    pub typ: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub port_comms: Option<u16>,
    #[cfg_attr(feature = "hash", derivative(Hash = "ignore"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub properties: Option<HashMap<String, String>>,
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
pub struct Site {
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub created: OffsetDateTime,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub modified: Option<OffsetDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub internal_meta_data: Option<HashMap<String, String>>,
    pub name: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    pub location: Location,
    pub base_fps_port: i32,
    #[cfg_attr(feature = "serde", serde(default))]
    pub properties: Option<HashMap<String, String>>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "_links", with = "utils::links::serde", default)
    )]
    pub links: HashMap<String, Url>,
}

impl Hateoas for Site {
    fn get_links(&self) -> &HashMap<String, url::Url> {
        &self.links
    }

    fn get_links_mut(&mut self) -> &mut HashMap<String, url::Url> {
        &mut self.links
    }
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteConfiguration {
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub created: OffsetDateTime,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub modified: Option<OffsetDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub internal_meta_data: Option<HashMap<String, String>>,
    pub name: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    pub configuration_seconds: i32,
    #[cfg_attr(feature = "serde", serde(rename = "virtual", default))]
    pub virtual_cfg: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub properties: Option<HashMap<String, String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub hardware: Option<Vec<SiteHardware>>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "_links", with = "utils::links::serde", default)
    )]
    pub links: HashMap<String, Url>,
}

impl Hateoas for SiteConfiguration {
    fn get_links(&self) -> &HashMap<String, url::Url> {
        &self.links
    }

    fn get_links_mut(&mut self) -> &mut HashMap<String, url::Url> {
        &mut self.links
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    #[test]
    fn missing_option() {
        let string = r#"
        {
            "manual": true,
            "name": "Testing",
            "created": "2020-08-12T04:05:20Z"
        }
        "#;
        serde_json::from_str::<SiteHardware>(string).unwrap();
    }
}
