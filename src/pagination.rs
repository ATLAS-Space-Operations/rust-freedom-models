use std::collections::HashMap;

use crate::Hateoas;

use super::utils;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

/// A paginated response
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Paginated<T>
where
    T: DeserializeOwned,
{
    #[serde(rename = "_embedded")]
    #[serde(deserialize_with = "utils::destructure::serde::deserialize")]
    pub items: Vec<T>,
    #[serde(rename = "_links")]
    #[serde(with = "utils::links::serde")]
    pub links: HashMap<String, Url>,
    pub page: Page,
}

impl<T> Hateoas for Paginated<T>
where
    T: DeserializeOwned,
{
    fn get_links(&self) -> &HashMap<String, url::Url> {
        &self.links
    }

    fn get_links_mut(&mut self) -> &mut HashMap<String, url::Url> {
        &mut self.links
    }
}

/// Page metadata included in a paginated stream
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub size: u32,
    pub total_elements: u32,
    pub total_pages: u32,
    pub number: u32,
}
