use anyhow::Result;
use async_trait::async_trait;
use reqwest::header::HeaderMap;

use crate::{auth::AuthClient, Query};

use super::Oauth2Info;

/// Client to handle Oauth2.
/// This client is implements `OauthClient` trait and add bearer token to every request.
pub struct Oauth2Client {
    client: reqwest::Client,
    credentials: Oauth2Info,
    base_url: String,
}
impl Oauth2Client {
    pub fn new(credentials: Oauth2Info, base_url: &str) -> Oauth2Client {
        Oauth2Client {
            client: reqwest::Client::new(),
            credentials,
            base_url: base_url.to_string(),
        }
    }
    fn generate_header(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.append(
            "Authorization",
            format!("Bearer {}", self.credentials.bearer).parse()?,
        );
        Ok(headers)
    }
}
#[async_trait]
impl AuthClient for Oauth2Client {
    async fn get(&self, url: &str, query: Query) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, url);
        let headers = self.generate_header();
        let res = self
            .client
            .get(url)
            .query(&query)
            .headers(headers?)
            .send()
            .await?;
        Ok(res)
    }
    async fn post(&self, url: &str, query: Query) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, url);
        let headers = self.generate_header();
        Ok(self
            .client
            .get(url)
            .form(&query)
            .headers(headers?)
            .send()
            .await
            .unwrap())
    }
}
