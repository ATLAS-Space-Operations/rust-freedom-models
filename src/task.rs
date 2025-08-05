#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{AsRefStr, EnumString};
use time::OffsetDateTime;
use url::Url;

use crate::Hateoas;

#[cfg(feature = "serde")]
use super::utils;

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRefStr, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum TaskStatusType {
    Received,
    Pending,
    Scheduled,
    Rejected,
    Denied,
    MovedVis,
    Bumped,
    Moved,
    Cancelled,
    SystemError,
    QueuedPass,
    Configured,
    Downlinking,
    Recording,
    CompletedPass,
    DataCloud,
    Processing,
    ReadyCustom,
    ProcessedCustom,
    ErrorCustom,
    Completed,
    CompletedError,
    PushedToCustomer,
    ErrorPushToCustomer,
    Invoiced,
    Paid,
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskStatus {
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::timestamp"))]
    pub created: OffsetDateTime,
    pub status: TaskStatusType,
    pub reason: String,
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
#[derive(
    Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRefStr, EnumString,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum Polarization {
    #[default]
    Right,
    Left,
    Vertical,
    Horizontal,
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsRefStr, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum TaskType {
    Before,
    After,
    Test,
    Around,
    Exact,
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub created: OffsetDateTime,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub modified: Option<OffsetDateTime>,
    pub found_visibility: bool,
    /// Unavailable for user accounts
    #[cfg_attr(feature = "serde", serde(default))]
    pub internal_meta_data: Option<HashMap<String, String>>,
    /// Unavailable for user accounts
    #[cfg_attr(feature = "serde", serde(default))]
    pub score: Option<u8>,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub start: OffsetDateTime,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub end: OffsetDateTime,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub visibility_start: Option<OffsetDateTime>,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub visibility_end: Option<OffsetDateTime>,
    pub billable: bool,
    pub duration_in_seconds: u32,
    pub task_within_config_window: bool,
    pub duration: String,
    pub file_results: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub meta_data: Option<HashMap<String, String>>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "_links", with = "utils::links::serde", default)
    )]
    pub links: HashMap<String, Url>,
}

impl Hateoas for Task {
    fn get_links(&self) -> &HashMap<String, url::Url> {
        &self.links
    }

    fn get_links_mut(&mut self) -> &mut HashMap<String, url::Url> {
        &mut self.links
    }
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskStatusEvent {
    pub task_request_id: i32,
    pub task_request_uri: String,
    pub status_changes: Vec<TaskStatus>,
}

impl TaskStatusEvent {
    pub fn latest_status(&self) -> Option<&TaskStatus> {
        self.status_changes
            .iter()
            .max_by_key(|status| status.created)
    }
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskRequest {
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub created: OffsetDateTime,
    #[cfg_attr(
        feature = "serde",
        serde(default, with = "time::serde::iso8601::option")
    )]
    pub modified: Option<OffsetDateTime>,
    /// Unavailable for user accounts
    #[cfg_attr(feature = "serde", serde(default))]
    pub internal_meta_data: Option<HashMap<String, String>>,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub task_type: TaskType,
    #[cfg_attr(feature = "serde", serde(default))]
    pub hours_of_flex: u32,
    pub duration: u32,
    pub minimum_duration: u32,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub target_date: OffsetDateTime,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub earliest_start: OffsetDateTime,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::iso8601"))]
    pub latest_start: OffsetDateTime,
    pub transmitting: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub test_file: Option<String>,
    pub status_changes: Vec<TaskStatus>,
    pub task_active: bool,
    pub task_request_scheduled: bool,
    pub task_request_cancelled: bool,
    pub flex: bool,
    pub latest_status_change: TaskStatus,
    #[cfg_attr(feature = "serde", serde(default))]
    pub meta_data: Option<HashMap<String, String>>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "_links", with = "utils::links::serde", default)
    )]
    pub links: HashMap<String, Url>,
}

impl Hateoas for TaskRequest {
    fn get_links(&self) -> &HashMap<String, url::Url> {
        &self.links
    }

    fn get_links_mut(&mut self) -> &mut HashMap<String, url::Url> {
        &mut self.links
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "serde")]
    fn task_status_event() {
        let json = r#"
{
  "taskRequestId": 189348,
  "taskRequestUri": "https://test-api.atlasground.com/api/requests/189348",
  "statusChanges": [
    {
      "created": "2025-08-01T19:55:07.061Z",
      "status": "RECEIVED",
      "reason": "Saved to Database and awaiting scheduling"
    },
    {
      "created": "2025-08-01T19:55:07.674Z",
      "status": "SCHEDULED",
      "reason": "Test Task Scheduled"
    },
    {
      "created": "2025-08-01T19:55:08.986Z",
      "status": "QUEUED_PASS",
      "reason": "Pass has been queued to execute"
    }
  ]
}"#;
        let event: TaskStatusEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event.latest_status().unwrap().status,
            TaskStatusType::QueuedPass
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn task_status_event_float_timestamp() {
        use time::macros::datetime;

        let json = r#"
{
  "taskRequestId": 190029,
  "taskRequestUri": "https://test-api.atlasground.com/api/requests/190029",
  "statusChanges": [
    {
      "created": 1754409946.362000000,
      "status": "RECEIVED",
      "reason": "Saved to Database and awaiting scheduling"
    }
  ]
}"#;
        let event: TaskStatusEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            datetime!(2025 - 08 - 05 16:05:46.361_999_989).assume_utc(),
            event.status_changes[0].created
        );
    }
}
