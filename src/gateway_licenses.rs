//! # Gateway Licenses
//!
//! Contains models for interacting with Freedom Gateway licensing endpoints.

use strum::{AsRefStr, EnumString};
use time::OffsetDateTime;

/// Response body returned when regenerating a license key.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct RegenerateResponse {
    pub account_id: u64,
    pub license_id: u32,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub expires_at: OffsetDateTime,
    pub license_key: String,
}

/// Response body returned from a license verification request.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct VerifyResponse {
    pub valid: bool,
    pub license_id: Option<u32>,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub expires_at: Option<OffsetDateTime>,
    pub reason: Option<String>,
}

/// Response body for viewing all licenses associated with an account.
///
/// This is a wrapper type over a list of [`ViewOne`] items, corresponding to each license record
/// for the account.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct View(pub Vec<ViewOne>);

/// Representation of a single license associated with an account.
///
/// Used in license listing and detail-view responses.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct ViewOne {
    pub id: u32,
    pub account_id: u64,
    pub status: Status,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub expires_at: OffsetDateTime,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub last_used_at: Option<OffsetDateTime>,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub created: OffsetDateTime,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub modified: OffsetDateTime,
    pub key_version: u32,
}

/// The current status of a license.
///
/// Additional variants may be added in the future, so consumers should
/// handle this enum non-exhaustively.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRefStr, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub enum Status {
    /// License is active and can be used.
    Active,
    /// License is inactive and should not be used.
    Inactive,
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn view_all() {
        let json = r#"[
    {
        "id": 1,
        "accountId": 1,
        "status": "ACTIVE",
        "expiresAt": "2025-12-11T00:00:00Z",
        "lastUsedAt": "2025-12-09T23:15:10.520830Z",
        "created": "2025-12-09T23:14:22.482359Z",
        "modified": "2025-12-09T23:15:10.522040Z",
        "keyVersion": 1
    },
    {
        "id": 2,
        "accountId": 1,
        "status": "ACTIVE",
        "expiresAt": "2025-12-11T00:00:00Z",
        "created": "2025-12-10T17:31:59.709112Z",
        "modified": "2025-12-10T17:31:59.709112Z",
        "keyVersion": 1
    }
]"#;
        let view: View = serde_json::from_str(json).unwrap();
        let should_be = View(vec![
            ViewOne {
                id: 1,
                account_id: 1,
                status: Status::Active,
                expires_at: datetime!(2025 - 12 - 11 00:00:00).assume_utc(),
                last_used_at: Some(datetime!(2025 - 12 - 09 23:15:10.520_830).assume_utc()),
                created: datetime!(2025 - 12 - 09 23:14:22.482_359).assume_utc(),
                modified: datetime!(2025 - 12 - 09 23:15:10.522_040).assume_utc(),
                key_version: 1,
            },
            ViewOne {
                id: 2,
                account_id: 1,
                status: Status::Active,
                expires_at: datetime!(2025 - 12 - 11 00:00:00).assume_utc(),
                last_used_at: None,
                created: datetime!(2025 - 12 - 10 17:31:59.709_112).assume_utc(),
                modified: datetime!(2025 - 12 - 10 17:31:59.709_112).assume_utc(),
                key_version: 1,
            },
        ]);
        assert_eq!(view, should_be);
    }

    #[test]
    fn view_one() {
        let json = r#"
    {
        "id": 1,
        "accountId": 1,
        "status": "ACTIVE",
        "expiresAt": "2025-12-11T00:00:00Z",
        "lastUsedAt": "2025-12-09T23:15:10.520830Z",
        "created": "2025-12-09T23:14:22.482359Z",
        "modified": "2025-12-09T23:15:10.522040Z",
        "keyVersion": 1
    }"#;
        let view: ViewOne = serde_json::from_str(json).unwrap();
        let should_be = ViewOne {
            id: 1,
            account_id: 1,
            status: Status::Active,
            expires_at: datetime!(2025 - 12 - 11 00:00:00).assume_utc(),
            last_used_at: Some(datetime!(2025 - 12 - 09 23:15:10.520_830).assume_utc()),
            created: datetime!(2025 - 12 - 09 23:14:22.482_359).assume_utc(),
            modified: datetime!(2025 - 12 - 09 23:15:10.522_040).assume_utc(),
            key_version: 1,
        };
        assert_eq!(view, should_be);
    }

    #[test]
    fn view_one_missing_last_used() {
        let json = r#"
    {
        "id": 1,
        "accountId": 1,
        "status": "ACTIVE",
        "expiresAt": "2025-12-11T00:00:00Z",
        "created": "2025-12-09T23:14:22.482359Z",
        "modified": "2025-12-09T23:15:10.522040Z",
        "keyVersion": 1
    }"#;
        let view: ViewOne = serde_json::from_str(json).unwrap();
        let should_be = ViewOne {
            id: 1,
            account_id: 1,
            status: Status::Active,
            expires_at: datetime!(2025 - 12 - 11 00:00:00).assume_utc(),
            last_used_at: None,
            created: datetime!(2025 - 12 - 09 23:14:22.482_359).assume_utc(),
            modified: datetime!(2025 - 12 - 09 23:15:10.522_040).assume_utc(),
            key_version: 1,
        };
        assert_eq!(view, should_be);
    }

    #[test]
    fn regenerate_response() {
        let json = r#"{
    "licenseId": 1,
    "accountId": 1,
    "expiresAt": "2025-12-11T00:00:00Z",
    "licenseKey": "foobar"
}"#;
        let regenerate: RegenerateResponse = serde_json::from_str(json).unwrap();
        let should_be = RegenerateResponse {
            account_id: 1,
            license_id: 1,
            expires_at: datetime!(2025 - 12 - 11 00:00:00).assume_utc(),
            license_key: String::from("foobar"),
        };
        assert_eq!(regenerate, should_be);
    }

    #[test]
    fn verify_response_valid() {
        let json = r#"{
    "valid": true,
    "licenseId": 1,
    "expiresAt": "2025-12-11T00:00:00Z"
}"#;
        let verify: VerifyResponse = serde_json::from_str(json).unwrap();
        let should_be = VerifyResponse {
            valid: true,
            license_id: Some(1),
            expires_at: Some(datetime!(2025 - 12 - 11 00:00:00).assume_utc()),
            reason: None,
        };
        assert_eq!(verify, should_be);
    }

    #[test]
    fn verify_response_invalid() {
        let json = r#"{
    "valid": false,
    "reason": "INVALID"
}"#;
        let verify: VerifyResponse = serde_json::from_str(json).unwrap();
        let should_be = VerifyResponse {
            valid: false,
            license_id: None,
            expires_at: None,
            reason: Some(String::from("INVALID")),
        };
        assert_eq!(verify, should_be);
    }
}
