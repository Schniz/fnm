use crate::{pretty_serde::DecodeError, version::Version};
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

    impl From<LtsStatus> for Option<String> {
        fn from(status: LtsStatus) -> Self {
            match status {
                LtsStatus::Nope(_) => None,
                LtsStatus::Yes(x) => Some(x),
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

#[derive(Deserialize, Debug)]
pub struct IndexedNodeVersion {
    pub version: Version,
    #[serde(with = "lts_status")]
    pub lts: Option<String>,
    // pub date: chrono::NaiveDate,
    // pub files: Vec<String>,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    #[error("can't get remote versions file: {0}")]
    #[diagnostic(transparent)]
    Http(#[from] crate::http::Error),
    #[error("can't decode remote versions file: {0}")]
    #[diagnostic(transparent)]
    Decode(#[from] DecodeError),
}

/// Prints
///
/// ```rust
/// use crate::remote_node_index::list;
/// ```
pub fn list(base_url: &Url) -> Result<Vec<IndexedNodeVersion>, Error> {
    let base_url = base_url.as_str().trim_end_matches('/');
    let index_json_url = format!("{base_url}/index.json");
    let resp = crate::http::get(&index_json_url)
        .map_err(crate::http::Error::from)?
        .error_for_status()
        .map_err(crate::http::Error::from)?;
    let text = resp.text().map_err(crate::http::Error::from)?;
    let mut value: Vec<IndexedNodeVersion> =
        serde_json::from_str(&text[..]).map_err(|cause| DecodeError::from_serde(text, cause))?;
    value.sort_by(|a, b| a.version.cmp(&b.version));
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
