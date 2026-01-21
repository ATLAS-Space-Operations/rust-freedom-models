#![doc = include_str!("../README.md")]

pub mod account;
pub mod azel;
pub mod band;
pub mod error;
pub mod gateway_licenses;
#[cfg(feature = "serde")]
pub mod pagination;
pub mod satellite;
pub mod satellite_configuration;
pub mod site;
pub mod status;
pub mod task;
pub mod task_override;
pub mod user;
#[cfg(feature = "serde")]
pub mod utils;

/// A trait for navigating the Hateoas structure of Freedom models
pub trait Hateoas {
    fn get_links(&self) -> &std::collections::HashMap<String, url::Url>;

    fn get_links_mut(&mut self) -> &mut std::collections::HashMap<String, url::Url>;
}
