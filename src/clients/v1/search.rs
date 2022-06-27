use anyhow::Result;

use super::V1Client;

use crate::models::v1::search::*;

impl V1Client {
    pub async fn search_tweets(&self, q: &str) -> Result<SearchTweetsResponse> {
        let v: Vec<(String, String)> = vec![("q".to_string(), q.to_string())];
        let res = self
            .client
            .get("/search/tweets.json", v)
            .await?
            .text()
            .await?;
        dbg!(&res);
        let res = serde_json::from_str::<SearchTweetsResponse>(&res)?;
        Ok(res)
    }
}
