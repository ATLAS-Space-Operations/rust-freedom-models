#![doc = include_str!("../README.md")]

pub mod account;
pub mod azel;
pub mod band;
pub mod error;
#[cfg(feature = "serde")]
pub mod pagination;
pub mod satellite;
pub mod site;
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
