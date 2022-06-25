use async_trait::async_trait;
use std::collections::HashMap;

/// Trait that twitter api v1.1 client must meet
#[async_trait]
pub(super) trait ClientV1 {
    const BASE_URL: &'static str = "https://api.twitter.com/1.1";
    async fn get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error>;
    async fn post(
        &self,
        url: &str,
        body: HashMap<String, String>,
    ) -> Result<reqwest::Response, reqwest::Error>;
}

#[async_trait]
pub(super) trait ClientV2 {
    async fn get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error>;
    async fn post(
        &self,
        url: &str,
        body: HashMap<String, String>,
    ) -> Result<reqwest::Response, reqwest::Error>;
    async fn delete(&self, url: &str) -> Result<reqwest::Response, reqwest::Error>;
    async fn put(&self, url: &str) -> Result<reqwest::Response, reqwest::Error>;
}

use crate::models::v1;

#[async_trait]
pub(super) trait V1Read: ClientV1 {
    async fn search_tweets(
        &self,
        query: v1::search_tweets::Query,
    ) -> Result<v1::search_tweets::Response, reqwest::Error> {
        let url = format!("{}{}", Self::BASE_URL, "/search/tweets.json");
        let response: v1::search_tweets::Response = self.get(&url).await?.json().await?;
        Ok(response)
    }
}
