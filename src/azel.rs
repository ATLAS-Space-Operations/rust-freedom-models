#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub elevation: f64,
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
pub struct Direction {
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub timestamp: OffsetDateTime,
    pub az: f64,
    pub el: f64,
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
pub struct AzEl {
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub start: OffsetDateTime,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub end: OffsetDateTime,
    pub location: Location,
    pub directions: Vec<Direction>,
}
