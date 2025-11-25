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
    pub metrics: Vec<ValueMetric>,
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
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub value: Value,
    pub unit: String,
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
  "site": {
    "taskId": 1,
    "taskRequestUri": "https://test-api.atlasground.com/api/requests/1",
    "name": "MySite",
    "configuration": "MySite-000A",
    "siteUri": "https://test-api.atlasground.com/api/sites/1",
    "configUri": "https://test-api.atlasground.com/api/configurations/1",
    "collected": 1648733340.076,
    "hardwareMetrics": [
      {
        "name": "test-hardware",
        "mfg": "Tech",
        "model": "1.x.x",
        "type": "MODEM",
        "metrics": [
          {
            "type": "site.hardware.modem.status",
            "value": true,
            "unit": "bool",
            "valueType": "bool"
          }
        ]
      },
      {
        "name": "test-hardware",
        "mfg": "Tech",
        "model": "1.x.x",
        "type": "DIGITIZER",
        "metrics": [
          {
            "type": "site.hardware.digitizer.status",
            "value": true,
            "unit": "bool",
            "valueType": "bool"
          }
        ]
      },
      {
        "name": "Site Server",
        "mfg": "ATLAS",
        "model": "1.x.x",
        "type": "SUM",
        "metrics": [
          {
            "type": "site.hardware.status",
            "value": true,
            "unit": "bool",
            "valueType": "bool"
          },
          {
            "type": "site.hardware.pass.timing",
            "value": 3467,
            "unit": "s",
            "valueType": "long"
          }
        ]
      }
    ]
  }
}"#;

        let ser: PassMetric = serde_json::from_slice(json_value.as_bytes()).unwrap();
        let first = &ser.site.hardware_metrics[0].metrics[0];
        assert_eq!(first.type_string.as_str(), "site.hardware.modem.status");
        assert_eq!(&first.value, &Value::Bool(true));
        let second = &ser.site.hardware_metrics[1].metrics[0];
        assert_eq!(
            second.type_string.as_str(),
            "site.hardware.digitizer.status"
        );
        assert_eq!(&second.value, &Value::Bool(true));
        let second = &ser.site.hardware_metrics[1].metrics[0];
        assert_eq!(
            second.type_string.as_str(),
            "site.hardware.digitizer.status"
        );
        assert_eq!(&second.value, &Value::Bool(true));
        let third = &ser.site.hardware_metrics[2].metrics[0];
        assert_eq!(third.type_string.as_str(), "site.hardware.status");
        assert_eq!(&third.value, &Value::Bool(true));
        let fourth = &ser.site.hardware_metrics[2].metrics[1];
        assert_eq!(fourth.type_string.as_str(), "site.hardware.pass.timing");
        assert_eq!(&fourth.value, &Value::Long(3467));
    }
}
