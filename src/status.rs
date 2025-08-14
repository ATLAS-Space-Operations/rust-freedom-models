#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct PassMetric {
    pub site: SiteMetric,
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct SiteMetric {
    pub task_id: i32,
    pub name: String,
    pub task_request_uri: String,
    pub configuration: String,
    pub site_uri: String,
    pub config_uri: String,
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::timestamp"))]
    pub collected: OffsetDateTime,
    pub hardware_metrics: Vec<HardwareMetric>,
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct HardwareMetric {
    pub name: String,
    pub mfg: String,
    pub model: Option<String>,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub hw_type: String,
    pub metrics: Vec<Value>,
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct ValueMetric {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub type_string: String,
    pub value: Value,
    pub unit: String,
    pub value_type: String,
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase", tag = "valueType", content = "value")
)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Value {
    Bool(bool),
    Float(f64),
    Long(i64),
    Int(i32),
    String(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "serde")]
    fn deserialize() {
        let json_value = r#"{
     "site":
     {
         "taskId":78939,
         "taskRequestUri":"https://test-api.atlasground.com/api/requests/78939",
         "name":"AIB1",
         "configuration":"AIB1-000A",
         "siteUri":"https://test-api.atlasground.com/api/sites/27",
         "configUri":"https://test-api.atlasground.com/api/configurations/35",
         "collected":1648733340.076000000,
         "hardwareMetrics":
         [
             {
                 "name":"aib1_000a.modem.amergint_technologies.0",
                 "mfg":"Amergint Technologies",
                 "model":"satTRAC",
                 "type":"MODEM",
                 "metrics":
                 [
                     {
                         "type":"site.hardware.modem.status",
                         "value":true,
                         "unit":"bool",
                         "valueType":"bool"
                     }
                 ]
             },
             {
                 "name":"aib1_000a.digitizer.real_time_logic.0",
                 "mfg":"Real Time Logic",
                 "model":"1.7.4",
                 "type":"DIGITIZER",
                 "metrics":
                 [
                     {
                         "type":"site.hardware.digitizer.status",
                         "value":true,
                         "unit":"bool",
                         "valueType":"bool"
                     }
                 ]
             },
             {
                 "name":"Site Server",
                 "mfg":"ATLAS",
                 "model":"1.6.1",
                 "type":"SUM",
                 "metrics":
                 [
                     {
                         "type":"site.hardware.status",
                         "value":true,
                         "unit":"bool",
                         "valueType":"bool"
                     },
                     {
                         "type":"site.hardware.pass.timing",
                         "value":3467,
                         "unit":"s",
                         "valueType":"long"
                     }
                 ]
             }
         ]
     }
 }"#;

        let _ser: PassMetric = serde_json::from_slice(json_value.as_bytes()).unwrap();
    }
}
