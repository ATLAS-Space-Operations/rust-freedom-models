//! # Gateway Licenses
//!
//! Contains models for interacting with Freedom Gateway licensing endpoints.
use strum::{AsRefStr, EnumString};
use time::OffsetDateTime;

/// Response for regenerating a license key
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegenerateResponse {
    pub account_id: u32,
    pub license_id: u32,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub expires_at: OffsetDateTime,
    pub license_key: String,
}

/// Request body for verifying a license key
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Verify {
    pub license_key: String,
}

/// Response body for verifying a license key
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VerifyResponse {
    pub valid: bool,
    pub license_id: u32,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601::option"))]
    pub expires_at: Option<OffsetDateTime>,
    pub reason: Option<String>,
}

/// Response body viewing the list of licenses associated with your account
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct View(pub Vec<ViewOne>);

/// Response body viewing a single license associated with your account
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViewOne {
    pub id: u32,
    pub account_id: u32,
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

/// The current status of the license
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRefStr, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum Status {
    Active,
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
    fn verify_request() {
        let json = r#"{
    "licenseKey": "foobar"
}"#;
        let verify: Verify = serde_json::from_str(json).unwrap();
        let should_be = Verify {
            license_key: String::from("foobar"),
        };
        assert_eq!(verify, should_be);
    }

    #[test]
    fn verify_response() {
        let json = r#"{
    "valid": true,
    "licenseId": 1,
    "expiresAt": "2025-12-11T00:00:00Z"
}"#;
        let verify: VerifyResponse = serde_json::from_str(json).unwrap();
        let should_be = VerifyResponse {
            valid: true,
            license_id: 1,
            expires_at: Some(datetime!(2025 - 12 - 11 00:00:00).assume_utc()),
            reason: None,
        };
        assert_eq!(verify, should_be);
    }
}
