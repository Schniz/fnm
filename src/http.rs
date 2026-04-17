//! This module is an adapter for HTTP related operations.
//! In the future, if we want to migrate to a different HTTP library,
//! we can easily change this facade instead of multiple places in the crate.

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use reqwest::{blocking::Client, IntoUrl};

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
#[diagnostic(code("fnm::http::error"))]
pub struct Error(#[from] reqwest::Error);
pub type Response = reqwest::blocking::Response;

fn build_headers(auth_header: Option<&str>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(concat!("fnm ", env!("CARGO_PKG_VERSION"))),
    );
    if let Some(auth) = auth_header {
        if let Ok(value) = HeaderValue::from_str(auth) {
            headers.insert(AUTHORIZATION, value);
        }
    }
    headers
}

pub fn get(url: impl IntoUrl, auth_header: Option<&str>) -> Result<Response, Error> {
    Ok(Client::new()
        .get(url)
        .headers(build_headers(auth_header))
        .send()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_headers_without_auth() {
        let headers = build_headers(None);

        // User-Agent should always be set
        assert!(headers.contains_key(USER_AGENT));
        assert!(headers
            .get(USER_AGENT)
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("fnm "));

        // Authorization should not be set
        assert!(!headers.contains_key(AUTHORIZATION));
    }

    #[test]
    fn test_build_headers_with_auth() {
        let headers = build_headers(Some("Bearer token123"));

        // User-Agent should always be set
        assert!(headers.contains_key(USER_AGENT));

        // Authorization should be set with correct value
        assert!(headers.contains_key(AUTHORIZATION));
        assert_eq!(
            headers.get(AUTHORIZATION).unwrap().to_str().unwrap(),
            "Bearer token123"
        );
    }

    #[test]
    fn test_build_headers_with_basic_auth() {
        let headers = build_headers(Some("Basic abc123"));

        assert_eq!(
            headers.get(AUTHORIZATION).unwrap().to_str().unwrap(),
            "Basic abc123"
        );
    }

    #[test]
    fn test_build_headers_with_empty_auth() {
        let headers = build_headers(Some(""));

        // Empty string should still set the header (empty value is valid)
        assert!(headers.contains_key(AUTHORIZATION));
    }
}
