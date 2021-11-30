//! This module is an adapter for HTTP related operations.
//! In the future, if we want to migrate to a different HTTP library,
//! we can easily change this facade instead of multiple places in the crate.

pub type Error = reqwest::Error;
pub type Response = reqwest::blocking::Response;

pub fn get(url: &str) -> Result<Response, Error> {
    reqwest::blocking::get(url)
}
