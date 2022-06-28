use anyhow::Result;
use async_trait::async_trait;

use crate::{auth::AuthClient, Query};

use super::{utils::generate_header, Oauth1aInfo};

/// Client to handle Oauth1a.
/// This client implements `OauthClient` trait and add oauth signatures and so on for every
/// requests.
pub struct Oauth1aClient {
    client: reqwest::Client,
    credentials: Oauth1aInfo,
    base_url: String,
}
impl Oauth1aClient {
    pub fn new(credentials: Oauth1aInfo, base_url: &str) -> Oauth1aClient {
        Oauth1aClient {
            client: reqwest::Client::new(),
            credentials,
            base_url: base_url.to_string(),
        }
    }
}
#[async_trait]
impl AuthClient for Oauth1aClient {
    async fn get(&self, url: &str, query: Query) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, url);
        let headers = generate_header(
            "GET",
            &query,
            &url,
            &self.credentials.consumer_key,
            &self.credentials.consumer_secret,
            &self.credentials.token,
            &self.credentials.token_secret,
        )?;
        let request = self
            .client
            .get(&url)
            .query(&query)
            .headers(headers.clone())
            .build()
            .unwrap();
        dbg!(&request);
        let res = self
            .client
            .get(&url)
            .query(&query)
            .headers(headers)
            .send()
            .await?;
        Ok(res)
    }
    async fn post(&self, url: &str, query: Query) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, url);
        let headers = generate_header(
            "POST",
            &query,
            &url,
            &self.credentials.consumer_key,
            &self.credentials.consumer_secret,
            &self.credentials.token,
            &self.credentials.token_secret,
        );
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
