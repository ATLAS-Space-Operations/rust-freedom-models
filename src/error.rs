#[cfg(feature = "serde")]
use serde::Deserialize;

#[cfg_attr(
    feature = "serde",
    derive(Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub enum Error {
    PaginatedInner,
    PaginatedListMissing,
    Link(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PaginatedInner => {
                f.write_str("An item within the inner paginated structure failed to deserialize")
            }
            Error::PaginatedListMissing => {
                f.write_str("The inner list of paginated elements is missing")
            }
            Error::Link(inner) => write!(f, "The link map failed to deserialize: {inner}"),
        }
    }
}

impl core::error::Error for Error {}
