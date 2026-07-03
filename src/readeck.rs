// Copyright 2025
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error;
use std::fmt;
use std::io::Read;

use reqwest::{
    blocking::Client,
    header::{self, HeaderValue},
    Error as HttpError,
};
use serde::Serialize;
use url::Url;

#[derive(Debug)]
pub enum ReadeckError {
    Http(HttpError),
    UnsuccessfulStatus(reqwest::StatusCode, Option<String>),
}

pub type ReadeckResult<T> = Result<T, ReadeckError>;

impl From<HttpError> for ReadeckError {
    fn from(err: HttpError) -> ReadeckError {
        ReadeckError::Http(err)
    }
}

impl Error for ReadeckError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ReadeckError::Http(e) => Some(e),
            ReadeckError::UnsuccessfulStatus(..) => None,
        }
    }
}

impl fmt::Display for ReadeckError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadeckError::Http(e) => e.fmt(fmt),
            ReadeckError::UnsuccessfulStatus(status, body) => {
                write!(fmt, "unexpected HTTP status: {}", status)?;
                if let Some(body) = body {
                    writeln!(fmt)?;
                    body.fmt(fmt)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Serialize)]
struct CreateBookmarkRequest<'a> {
    url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    labels: Vec<&'a str>,
}

pub struct Readeck {
    base_url: Url,
    auth_token: String,
    client: Client,
}

/// Builds the `/api/bookmarks` endpoint URL from a Readeck instance's base URL,
/// regardless of whether the base URL has a trailing slash
/// or already includes a path.
fn bookmarks_endpoint(base_url: &Url) -> Url {
    let mut url = base_url.clone();
    {
        let mut segments = url
            .path_segments_mut()
            .expect("base URL cannot be a base for relative URLs");
        segments.pop_if_empty();
        segments.push("api");
        segments.push("bookmarks");
    }
    url
}

impl Readeck {
    pub fn new(base_url: Url, auth_token: &str, client: Client) -> Readeck {
        Readeck {
            base_url,
            auth_token: auth_token.to_string(),
            client,
        }
    }

    /// Creates a bookmark for the given URL.
    ///
    /// `tags` should be a comma-separated list of tags,
    /// which will be sent as labels on the bookmark.
    pub fn add(&self, url: &Url, title: Option<&str>, tags: Option<&str>) -> ReadeckResult<()> {
        let labels: Vec<&str> = tags
            .map(|tags| tags.split(',').map(str::trim).filter(|t| !t.is_empty()).collect())
            .unwrap_or_default();

        let request = CreateBookmarkRequest {
            url: url.as_str(),
            title,
            labels,
        };

        let endpoint = bookmarks_endpoint(&self.base_url);

        let body = serde_json::to_string(&request).expect("failed to serialize request");

        let mut response = self
            .client
            .post(endpoint)
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.auth_token))
                    .expect("auth token is not a valid header value"),
            )
            .header(header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header(header::ACCEPT, HeaderValue::from_static("application/json"))
            .body(body)
            .send()?;

        let status = response.status();
        if !status.is_success() {
            let mut body = String::new();
            let body = response.read_to_string(&mut body).ok().map(|_| body);
            return Err(ReadeckError::UnsuccessfulStatus(status, body));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_trailing_slash_no_path() {
        let base = Url::parse("http://linkmaxxing.tailce413c.ts.net:3000").unwrap();
        assert_eq!(
            bookmarks_endpoint(&base).as_str(),
            "http://linkmaxxing.tailce413c.ts.net:3000/api/bookmarks"
        );
    }

    #[test]
    fn trailing_slash() {
        let base = Url::parse("https://readeck.example.com/").unwrap();
        assert_eq!(
            bookmarks_endpoint(&base).as_str(),
            "https://readeck.example.com/api/bookmarks"
        );
    }

    #[test]
    fn no_trailing_slash_with_path() {
        let base = Url::parse("https://readeck.example.com/subpath").unwrap();
        assert_eq!(
            bookmarks_endpoint(&base).as_str(),
            "https://readeck.example.com/subpath/api/bookmarks"
        );
    }
}
