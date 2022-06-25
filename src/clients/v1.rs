use std::collections::HashMap;

use async_trait::async_trait;

use super::traits::{ClientV1, V1Read};

pub struct V1ReadWriteDMClient {
    client: reqwest::Client,
}

impl V1ReadWriteDMClient {
    fn new() -> V1ReadWriteDMClient {
        let client = reqwest::Client::new();
        V1ReadWriteDMClient { client }
    }
}

#[async_trait]
impl ClientV1 for V1ReadWriteDMClient {
    async fn get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.client.get(url).send().await
    }
    async fn post(
        &self,
        url: &str,
        body: HashMap<String, String>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client.get(url).send().await
    }
}
#[async_trait]
impl V1Read for V1ReadWriteDMClient {}
