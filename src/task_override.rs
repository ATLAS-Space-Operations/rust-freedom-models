use std::collections::HashMap;

use time::OffsetDateTime;
use url::Url;

use crate::Hateoas;

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct ConfigurationDetails {
    pub name: String,
    pub description: String,
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct SatelliteDetails {
    pub name: String,
    pub norad: u32,
    pub description: String,
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct Override {
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub created: OffsetDateTime,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub modified: Option<OffsetDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub name: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub properties: HashMap<String, String>,
    /// The satellite configuration details associated with the override
    #[cfg_attr(feature = "serde", serde(alias = "satellite_details"))]
    pub satellite_details: Option<SatelliteDetails>,
    /// The site configuration details associated with the override
    #[cfg_attr(feature = "serde", serde(alias = "configuration_details"))]
    pub configuration_details: Option<ConfigurationDetails>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "_links", with = "crate::utils::links::serde", default)
    )]
    pub links: HashMap<String, Url>,
}

impl Hateoas for Override {
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
    fn deserialize_override_full() {
        let json = r#"{
    "created": "2026-01-22T16:16:40.836788Z",
    "modified": "2026-01-22T16:16:40.836788Z",
    "name": "C-Test-1",
    "properties": {
        "my.prop": "20"
    },
    "satellite_details": {
        "name": "Rustacean Nation",
        "norad": 101,
        "description": "this_will_work.unwrap().unwrap().unwrap()"
    },
    "configuration_details": {
        "name": "MySiteConfig",
        "description": "It's a good one!"
    },
    "_links": {
        "self": {
            "href": "http://localhost:8080/api/overrides/233"
        },
        "overrides": {
            "href": "http://localhost:8080/api/overrides/233"
        },
        "user": {
            "href": "http://localhost:8080/api/overrides/233/user"
        },
        "satellite": {
            "href": "http://localhost:8080/api/overrides/233/satellite"
        },
        "configuration": {
            "href": "http://localhost:8080/api/overrides/233/configuration"
        }
    }
}"#;

        let ov: Override = serde_json::from_str(json).unwrap();
        assert_eq!(ov.satellite_details.unwrap().name, "Rustacean Nation");
        assert_eq!(ov.configuration_details.unwrap().name, "MySiteConfig");
    }

    #[test]
    fn deserialize_override_minimal() {
        let json = r#"{
    "created": "2026-01-22T16:16:40.836788Z",
    "modified": "2026-01-22T16:16:40.836788Z",
    "name": "C-Test-1",
    "properties": {
        "my.prop": "20"
    },
    "_links": {
        "self": {
            "href": "http://localhost:8080/api/overrides/233"
        },
        "overrides": {
            "href": "http://localhost:8080/api/overrides/233"
        },
        "user": {
            "href": "http://localhost:8080/api/overrides/233/user"
        }
    }
}"#;

        let ov: Override = serde_json::from_str(json).unwrap();
        assert_eq!(ov.name, "C-Test-1");
        assert_eq!(ov.properties.get("my.prop").unwrap(), "20");
    }
}
