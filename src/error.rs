#[cfg(feature = "serde")]
use serde::Deserialize;
use thiserror::Error;

#[cfg_attr(
    feature = "serde",
    derive(Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum Error {
    #[error("An item within the inner paginated structure failed to deserialize")]
    PaginatedInner,

    #[error("The inner list of paginated elements is missing")]
    PaginatedListMissing,

    #[error("The link map failed to deserialize: {0}")]
    Link(String),
}
