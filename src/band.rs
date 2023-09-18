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
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BandType {
    Transmit,
    Receive,
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IoHardware {
    Modem,
    Fep,
    Cortex,
    Recorder,
    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
pub struct IoConfiguration {
    #[cfg_attr(feature = "serde", serde(default))]
    start_hex_pattern: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    end_hex_pattern: Option<String>,
    strip_pattern: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    io_hardware: Option<IoHardware>,
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
pub struct Band {
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub created: OffsetDateTime,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub modified: Option<OffsetDateTime>,
    pub name: String,
    #[cfg_attr(feature = "serde", serde(rename = "type", default))]
    pub typ: Option<BandType>,
    pub frequency_mghz: f64,
    pub default_band_width_mghz: f64,
    pub io_configuration: IoConfiguration,
    #[cfg_attr(feature = "serde", serde(default))]
    pub manual_transmit_control: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub account_name: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "_links", with = "utils::links::serde", default)
    )]
    pub links: HashMap<String, Url>,
}

impl Hateoas for Band {
    fn get_links(&self) -> &HashMap<String, url::Url> {
        &self.links
    }

    fn get_links_mut(&mut self) -> &mut HashMap<String, url::Url> {
        &mut self.links
    }
}
