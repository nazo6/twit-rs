use anyhow::Result;

use super::V1Client;
use serde::Deserialize;

use crate::models::v1::search::*;

impl V1Client {
    pub async fn search_tweets(&self, q: &str) -> Result<SearchTweetsResponse> {
        let v: Vec<(String, String)> = vec![("q".to_string(), q.to_string())];
        let res: SearchTweetsResponse = self
            .client
            .get("/search/tweets.json", v)
            .await?
            .json()
            .await?;
        Ok(res)
    }
}
