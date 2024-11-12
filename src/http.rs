//! This module is an adapter for HTTP related operations.
//! In the future, if we want to migrate to a different HTTP library,
//! we can easily change this facade instead of multiple places in the crate.

use reqwest::{blocking::Client, IntoUrl};

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
#[diagnostic(code("fnm::http::error"))]
pub struct Error(#[from] reqwest::Error);
pub type Response = reqwest::blocking::Response;

pub fn get(url: impl IntoUrl) -> Result<Response, Error> {
    Ok(Client::new()
        .get(url)
        // Some sites require a user agent.
        .header("User-Agent", concat!("fnm ", env!("CARGO_PKG_VERSION")))
        .send()?)
}
