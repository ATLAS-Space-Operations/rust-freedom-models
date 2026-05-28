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
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct UserPreferences {
    pub visibility_days: u32,
    pub min_elevation: f32,
    pub max_elevation: f32,
    pub min_duration: f32,
    pub elevation_tolerance: f32,
    pub duration_tolerance: f32,
    pub notify_via_email: bool,
    pub notify_via_text: bool,
}

/// The data model returned from the `api/users/search/whoami` endpoint
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct WhoAmI {
    /// ID of the user
    pub id: i32,
    /// First name of the user
    pub first_name: String,
    /// Last name of the user
    pub last_name: String,
    /// Email address of the user
    pub email: String,
    /// Describes whether the associated user was created as a machine service user
    pub machine_service: bool,
    /// The role assigned to the user
    pub ui_role: String,
    /// The set of privileges assigned to the user
    pub ui_privilege: String,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "_links", with = "utils::links::serde", default)
    )]
    pub links: HashMap<String, Url>,
}

impl Hateoas for WhoAmI {
    fn get_links(&self) -> &HashMap<String, url::Url> {
        &self.links
    }

    fn get_links_mut(&mut self) -> &mut HashMap<String, url::Url> {
        &mut self.links
    }
}

/// A user in freedom
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "unstable"), non_exhaustive)]
pub struct User {
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub created: OffsetDateTime,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub modified: Option<OffsetDateTime>,
    pub first_name: String,
    pub last_name: String,
    pub verified: bool,
    pub email: String,
    pub preferences: UserPreferences,
    /// Unavailable for user accounts
    #[cfg_attr(feature = "serde", serde(default))]
    pub internal_meta_data: Option<HashMap<String, String>>,
    /// Unavailable for user accounts
    #[cfg_attr(feature = "serde", serde(default))]
    pub deleted: Option<bool>,
    pub api_access_enabled: bool,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "_links", with = "utils::links::serde", default)
    )]
    pub links: HashMap<String, Url>,
}

impl Hateoas for User {
    fn get_links(&self) -> &HashMap<String, url::Url> {
        &self.links
    }

    fn get_links_mut(&mut self) -> &mut HashMap<String, url::Url> {
        &mut self.links
    }
}

#[cfg(all(feature = "serde", test))]
mod tests {
    use super::*;

    #[test]
    fn whoami_deserializes_full_response() {
        let json = r#"{
            "uiRole": "ROLE_OPERATOR",
            "uiPrivilege": "UI_PRIVILEGE_VIEW_TASKS,UI_PRIVILEGE_VIEW_SATELLITES",
            "id": 42,
            "email": "test.user@example.com",
            "firstName": "Test",
            "lastName": "User",
            "machineService": false,
            "_links": {
                "self": { "href": "https://api.example.com/api/users/42" },
                "users": { "href": "https://api.example.com/api/users/42" }
            }
        }"#;

        let whoami: WhoAmI = serde_json::from_str(json).unwrap();

        assert_eq!(whoami.id, 42);
        assert_eq!(whoami.email, "test.user@example.com");
        assert_eq!(whoami.first_name, "Test");
        assert_eq!(whoami.last_name, "User");
        assert!(!whoami.machine_service);
        assert_eq!(whoami.ui_role, "ROLE_OPERATOR");
        assert_eq!(
            whoami.ui_privilege,
            "UI_PRIVILEGE_VIEW_TASKS,UI_PRIVILEGE_VIEW_SATELLITES"
        );
    }

    #[test]
    fn whoami_deserializes_links() {
        let json = r#"{
            "uiRole": "ROLE_ADMIN",
            "uiPrivilege": "UI_PRIVILEGE_VIEW_ALL",
            "id": 1,
            "email": "admin@example.com",
            "firstName": "Admin",
            "lastName": "User",
            "machineService": false,
            "_links": {
                "self": { "href": "https://api.example.com/api/users/1" },
                "users": { "href": "https://api.example.com/api/users/1" }
            }
        }"#;

        let whoami: WhoAmI = serde_json::from_str(json).unwrap();

        let self_link = whoami.get_links().get("self").unwrap();
        assert_eq!(self_link.as_str(), "https://api.example.com/api/users/1");
        assert!(whoami.get_links().contains_key("users"));
    }

    #[test]
    fn whoami_deserializes_without_links() {
        let json = r#"{
            "uiRole": "ROLE_USER",
            "uiPrivilege": "UI_PRIVILEGE_VIEW_TASKS",
            "id": 99,
            "email": "nolinks@example.com",
            "firstName": "No",
            "lastName": "Links",
            "machineService": false
        }"#;

        let whoami: WhoAmI = serde_json::from_str(json).unwrap();
        assert!(whoami.links.is_empty());
    }

    #[test]
    fn whoami_deserializes_machine_service_user() {
        let json = r#"{
            "uiRole": "ROLE_SERVICE",
            "uiPrivilege": "UI_PRIVILEGE_MANAGE_TASKS",
            "id": 200,
            "email": "machine@example.com",
            "firstName": "Machine",
            "lastName": "Service",
            "machineService": true
        }"#;

        let whoami: WhoAmI = serde_json::from_str(json).unwrap();
        assert!(whoami.machine_service);
    }

    #[test]
    fn whoami_serializes_links_with_href_wrapper() {
        let json = r#"{
            "uiRole": "ROLE_USER",
            "uiPrivilege": "UI_PRIVILEGE_VIEW_TASKS",
            "id": 10,
            "email": "roundtrip@example.com",
            "firstName": "Round",
            "lastName": "Trip",
            "machineService": false,
            "_links": {
                "self": { "href": "https://api.example.com/api/users/10" }
            }
        }"#;

        let whoami: WhoAmI = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_value(&whoami).unwrap();

        assert_eq!(
            serialized["_links"]["self"]["href"],
            "https://api.example.com/api/users/10"
        );
    }

    #[test]
    fn whoami_get_links_mut_allows_insertion() {
        let json = r#"{
            "uiRole": "ROLE_USER",
            "uiPrivilege": "UI_PRIVILEGE_VIEW_TASKS",
            "id": 5,
            "email": "mut@example.com",
            "firstName": "Mut",
            "lastName": "User",
            "machineService": false
        }"#;

        let mut whoami: WhoAmI = serde_json::from_str(json).unwrap();
        whoami.get_links_mut().insert(
            "self".into(),
            "https://api.example.com/api/users/5".parse().unwrap(),
        );

        assert!(whoami.get_links().contains_key("self"));
    }
}
