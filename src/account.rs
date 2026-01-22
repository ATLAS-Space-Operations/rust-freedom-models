use crate::Hateoas;
use ipnet::Ipv4Net;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_with::serde_as;
use std::collections::HashMap;
use time::OffsetDateTime;
use url::Url;

#[cfg(feature = "serde")]
use super::utils;

/// Account CIDR
///
/// An IP range indicating the whitelisted IP address for connecting to the ATLAS FPS
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct AccountCidr {
    pub name: String,
    pub cidr: Ipv4Net,
}

impl std::ops::Deref for AccountCidr {
    type Target = Ipv4Net;

    fn deref(&self) -> &Self::Target {
        &self.cidr
    }
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct Tier {
    pub tier: u32,
    pub price: f32,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "_links", with = "utils::links::serde", default)
    )]
    pub links: HashMap<String, Url>,
}

impl Hateoas for Tier {
    fn get_links(&self) -> &HashMap<String, url::Url> {
        &self.links
    }

    fn get_links_mut(&mut self) -> &mut HashMap<String, url::Url> {
        &mut self.links
    }
}

#[cfg_attr(
    feature = "serde",
    serde_as,
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct Account {
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
    pub storage_key: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub storage_group: Option<String>,
    pub tiers: Vec<Tier>,
    pub post_process_done_by_account: bool,
    pub weeks_of_data_storage: u32,
    pub verified: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde_as(as = "VecSkipError<_>"))]
    pub access_realtime_cidr: Vec<AccountCidr>,
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde_as(as = "VecSkipError<_>"))]
    pub access_api_cidr: Vec<AccountCidr>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub external_id: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub fps_host_name: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "_links", with = "utils::links::serde", default)
    )]
    pub links: HashMap<String, Url>,
}

impl Hateoas for Account {
    fn get_links(&self) -> &HashMap<String, url::Url> {
        &self.links
    }

    fn get_links_mut(&mut self) -> &mut HashMap<String, url::Url> {
        &mut self.links
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    #[cfg(feature = "serde")]
    #[test]
    fn ip_cidr_deserialize() {
        let json = serde_json::json!({
            "name": "Test Account",
            "cidr": "192.168.1.96/28"
        });

        let cidr: AccountCidr = serde_json::from_value(json).unwrap();
        assert_eq!(cidr.name, "Test Account");
        assert_eq!(
            cidr.cidr,
            Ipv4Net::new_assert(Ipv4Addr::new(192, 168, 1, 96), 28)
        );
    }

    #[test]
    fn ip_cidr_check_28() {
        let cidr = AccountCidr {
            name: "Test".into(),
            cidr: Ipv4Net::new_assert(Ipv4Addr::new(192, 168, 1, 96), 28),
        };

        assert!(cidr.contains(&Ipv4Addr::new(192, 168, 1, 99)));
        assert!(cidr.contains(&Ipv4Addr::new(192, 168, 1, 97)));
        assert!(!cidr.contains(&Ipv4Addr::new(192, 168, 0, 99)));
    }

    #[test]
    fn ip_cidr_check_32() {
        let cidr = AccountCidr {
            name: "Test".into(),
            cidr: Ipv4Net::new_assert(Ipv4Addr::new(192, 168, 1, 96), 32),
        };

        assert!(cidr.contains(&Ipv4Addr::new(192, 168, 1, 96)));
        assert!(!cidr.contains(&Ipv4Addr::new(192, 168, 1, 97)));
        assert!(!cidr.contains(&Ipv4Addr::new(192, 168, 1, 95)));
        assert!(!cidr.contains(&Ipv4Addr::new(0, 0, 0, 0)));
    }
}
