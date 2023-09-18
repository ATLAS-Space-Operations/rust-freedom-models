use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Deserializer};
use url::Url;

use crate::Hateoas;

pub fn flatten_deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de> + 'de,
    T: DeserializeOwned,
{
    let vec_of_results: Vec<Result<T, String>> = Vec::deserialize(deserializer)?;

    Ok(vec_of_results.into_iter().flatten().collect())
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Embedded<T>
where
    T: DeserializeOwned,
{
    #[serde(rename = "_embedded")]
    #[serde(deserialize_with = "destructure::serde::deserialize")]
    pub items: T,
    #[serde(rename = "_links")]
    #[serde(with = "links::serde", default)]
    pub links: HashMap<String, Url>,
}

impl<T> Hateoas for Embedded<T>
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

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Content<T>
where
    T: DeserializeOwned + Hateoas,
{
    #[serde(flatten, deserialize_with = "content::serde::deserialize")]
    pub inner: T,
}

pub(crate) mod content {
    pub(crate) mod serde {
        use std::collections::HashMap;

        use serde::{de::DeserializeOwned, Deserialize, Deserializer};

        use crate::Hateoas;

        #[derive(Debug, Clone, Deserialize, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContentInitial<T> {
            #[serde(rename = "content")]
            pub inner: T,
            #[serde(rename = "_links")]
            #[serde(with = "crate::utils::links::serde", default)]
            pub links: HashMap<String, url::Url>,
        }

        pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
        where
            D: Deserializer<'de>,
            T: DeserializeOwned + Hateoas,
        {
            use serde::de::Error;

            let mut item_map: ContentInitial<T> = ContentInitial::<T>::deserialize(deserializer)
                .map_err(|e| Error::custom(format!("Failed to deserialize initial: {e}")))?;

            *(item_map.inner.get_links_mut()) = item_map.links;
            Ok(item_map.inner)
        }

        // Currently there is no reason to write serialization logic for paginated items.
    }
}

pub(crate) mod links {
    pub(crate) mod serde {
        use std::collections::HashMap;
        use url::Url;

        use serde::{Deserialize, Deserializer, Serialize, Serializer};

        #[derive(Serialize, Deserialize)]
        struct Link {
            href: Url,
        }

        pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Url>, D::Error>
        where
            D: Deserializer<'de>,
        {
            use std::collections::hash_map::RandomState;

            let link_map: HashMap<String, Link, RandomState> = HashMap::deserialize(deserializer)?;

            Ok(link_map
                .into_iter()
                .map(|(k, v)| (k, v.href))
                .collect::<HashMap<_, _>>())
        }

        pub(crate) fn serialize<S>(
            val: &HashMap<String, Url>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            use serde::ser::SerializeMap;

            let mut map = serializer.serialize_map(Some(val.len()))?;
            for (k, v) in val {
                let link = Link { href: v.to_owned() };
                map.serialize_entry(k, &link)?;
            }
            map.end()
        }
    }
}

/// This flattens a map, deserializing it based on the first value.
pub(crate) mod destructure {
    pub(crate) mod serde {
        use std::collections::HashMap;

        use serde::{de::DeserializeOwned, Deserialize, Deserializer};

        pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
        where
            D: Deserializer<'de>,
            T: DeserializeOwned,
        {
            use serde::de::Error;
            use std::collections::hash_map::RandomState;

            let item_map: HashMap<String, T, RandomState> = HashMap::deserialize(deserializer)?;

            item_map
                .into_iter()
                .next()
                .map(|(_, v)| v)
                .ok_or(Error::custom("Missing inner item list"))
        }

        // Currently there is no reason to write serialization logic for paginated items.
    }
}

#[cfg(test)]
mod test {
    use ::time::{format_description::well_known::Iso8601, OffsetDateTime, PrimitiveDateTime};

    #[test]
    fn time_deserialization() {
        let fmt = OffsetDateTime::now_utc().format(&Iso8601::DEFAULT);
        eprintln!("{:?}", fmt);
        let t1 = "2020-08-12T04:05:20Z";
        assert!(PrimitiveDateTime::parse(t1, &Iso8601::DEFAULT).is_ok());
        let t2 = "2020-08-12T04:05:20.040Z";
        assert!(PrimitiveDateTime::parse(t2, &Iso8601::DEFAULT).is_ok());
    }
}
