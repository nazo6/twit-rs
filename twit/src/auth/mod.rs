//! Module to generate auth credential.
//!
//! There are two authentication method for twitter api.
//! Oauth1.0a is traditional way. It can access to both v1.1 and v2.
//! Oauth2 is new way. It only can access to twitter api v2 but it can set scope of app. Oauth2 is
//! also used for app-only authentication.

use anyhow::Result;
use async_trait::async_trait;

use crate::Query;

pub mod oauth1a;
pub mod oauth2;
pub mod xauth;

#[async_trait]
pub(crate) trait AuthClient {
    async fn get(&self, url: &str, query: Query) -> Result<reqwest::Response>;
    async fn post(&self, url: &str, query: Query) -> Result<reqwest::Response>;
}
