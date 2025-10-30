#![doc = include_str!("../README.md")]

pub mod account;
pub mod azel;
pub mod band;
#[cfg(feature = "bundles")]
pub mod bundle;
pub mod error;
#[cfg(feature = "serde")]
pub mod pagination;
pub mod satellite;
pub mod satellite_configuration;
pub mod site;
pub mod status;
pub mod task;
pub mod user;
#[cfg(feature = "serde")]
pub mod utils;

use std::collections::HashMap;

/// A trait for navigating the Hateoas structure of Freedom models
pub trait Hateoas {
    fn get_links(&self) -> &HashMap<String, url::Url>;

    fn get_links_mut(&mut self) -> &mut HashMap<String, url::Url>;
}
