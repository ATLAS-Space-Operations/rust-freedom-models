use crate::Hateoas;
use cidr::Ipv4Cidr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_with::serde_as;
use std::collections::HashMap;
use time::OffsetDateTime;
use url::Url;

#[cfg(feature = "serde")]
use super::utils;

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
pub struct AccountCidr {
    pub name: String,
    pub cidr: Ipv4Cidr,
}

impl std::ops::Deref for AccountCidr {
    type Target = Ipv4Cidr;

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
    /// The name of the S3 bucket for the AWS child account associated with this account. Primarily
    /// available in Test and Prod.
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
