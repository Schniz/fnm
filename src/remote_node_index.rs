use std::io;

use crate::version::Version;
use brotli_decompressor::Decompressor;
use serde::Deserialize;
use url::Url;

mod lts_status {
    use serde::{Deserialize, Deserializer};

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    #[serde(untagged)]
    enum LtsStatus {
        Nope(bool),
        Yes(String),
    }

    impl Into<Option<String>> for LtsStatus {
        fn into(self) -> Option<String> {
            match self {
                Self::Nope(_) => None,
                Self::Yes(x) => Some(x),
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(LtsStatus::deserialize(deserializer)?.into())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[derive(Deserialize)]
        struct TestSubject {
            #[serde(deserialize_with = "deserialize")]
            lts: Option<String>,
        }

        #[test]
        fn test_false_deserialization() {
            let json = serde_json::json!({ "lts": false });
            let subject: TestSubject =
                serde_json::from_value(json).expect("Can't deserialize json");
            assert_eq!(subject.lts, None);
        }

        #[test]
        fn test_value_deserialization() {
            let json = serde_json::json!({ "lts": "dubnium" });
            let subject: TestSubject =
                serde_json::from_value(json).expect("Can't deserialize json");
            assert_eq!(subject.lts, Some("dubnium".into()));
        }
    }
}

#[derive(Deserialize, Debug, Eq, Ord)]
pub struct IndexedNodeVersion {
    pub version: Version,
    #[serde(with = "lts_status")]
    pub lts: Option<String>,
    pub date: chrono::NaiveDate,
    pub files: Vec<String>,
}

impl PartialEq for IndexedNodeVersion {
    fn eq(&self, other: &Self) -> bool {
        self.version.eq(&other.version)
    }
}

impl PartialOrd for IndexedNodeVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

/// Prints
///
/// ```rust
/// use crate::remote_node_index::list;
/// ```
pub fn list(base_url: &Url) -> Result<Vec<IndexedNodeVersion>, ureq::Error> {
    let index_json_url = format!("{}/index.json", base_url);
    let resp = ureq::get(&index_json_url)
        .set("Accept-Encoding", "br")
        .call()?;

    let brotli_encoded = resp
        .header("Content-Encoding")
        .map(|enc| enc.eq_ignore_ascii_case("br"))
        .unwrap_or_default();
    let reader = resp.into_reader();

    let value = if brotli_encoded {
        serde_json::from_reader(Decompressor::new(reader, 0))
    } else {
        serde_json::from_reader(reader)
    };

    let mut value: Vec<IndexedNodeVersion> = value.map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("failed decoding index JSON data: {}", e),
        )
    })?;

    value.sort();
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_list() {
        let base_url = Url::parse("https://nodejs.org/dist").unwrap();
        let expected_version = Version::parse("12.0.0").unwrap();
        let mut versions = list(&base_url).expect("Can't get HTTP data");
        assert_eq!(
            versions
                .drain(..)
                .find(|x| x.version == expected_version)
                .map(|x| x.version),
            Some(expected_version)
        );
    }
}
