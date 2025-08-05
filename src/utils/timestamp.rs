use serde::{Deserialize, Deserializer};
use time::OffsetDateTime;

#[allow(unused)] // So that we can use this as the serializer when we pass this module to serde
pub use time::serde::iso8601::serialize;

#[derive(Deserialize)]
#[serde(untagged)]
enum Inner {
    Float(f64),
    Timestamp(#[serde(deserialize_with = "time::serde::iso8601::deserialize")] OffsetDateTime),
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let float = match Inner::deserialize(deserializer)? {
        Inner::Float(float) => float,
        Inner::Timestamp(t) => return Ok(t),
    };

    let timestamp = OffsetDateTime::UNIX_EPOCH;
    let duration = time::Duration::saturating_seconds_f64(float);
    Ok(timestamp.saturating_add(duration))
}
