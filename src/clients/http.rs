use anyhow::Result;
use async_trait::async_trait;
use reqwest::header::HeaderMap;
use urlencoding::encode;

use crate::auth::{oauth1a::Oauth1aInfo, oauth2::Oauth2Info};

use super::Query;

#[async_trait]
pub trait OauthClient {
    async fn get(&self, url: &str, query: Query) -> Result<reqwest::Response>;
    async fn post(&self, url: &str, query: Query) -> Result<reqwest::Response>;
}

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
    fn generate_sig(&self, url: &str, query: Query, method: &str) -> String {
        let key = format!(
            "{}&{}",
            encode(&self.credentials.token_secret),
            encode(&self.credentials.consumer_secret)
        );

        let mut query = query;
        query.sort_by(|a, b| a.0.cmp(&b.0));
        let query = query
            .iter()
            .map(|q| format!("{}={}", q.0, q.1))
            .collect::<String>();
        let query = encode(&query);
        let method = encode(method);
        let url = encode(url);
        let data = format!("{method}&{url}&{query}");

        let hash = hmacsha1::hmac_sha1(key.as_bytes(), data.as_bytes());
        base64::encode(&hash)
    }
    fn generate_header(&self, method: &str, query: Query, url: &str) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        let timestamp = chrono::Utc::now().timestamp().to_string();

        headers.append("Authorization", "Oauth".parse()?);
        headers.append("oauth_nonce", timestamp.clone().parse()?);
        headers.append("oauth_timestamp", timestamp.parse()?);
        headers.append("oauth_consumer_key", self.credentials.consumer_key.parse()?);
        headers.append("oauth_signature_method", "HMAC-SHA1".parse()?);
        headers.append("oauth_version", "1.0".parse()?);
        headers.append("oauth_token", self.credentials.token.parse()?);
        headers.append(
            "oauth_signature",
            self.generate_sig(url, query, method).parse()?,
        );

        Ok(headers)
    }
}
#[async_trait]
impl OauthClient for Oauth1aClient {
    async fn get(&self, url: &str, query: Query) -> Result<reqwest::Response> {
        let mut url = format!("{}{}", self.base_url, url);
        query.iter().for_each(|q| {
            url = format!("{}&{}={}", url, q.0, q.1);
        });
        let headers = self.generate_header("GET", query, &format!("{}{}", self.base_url, url));
        Ok(self.client.get(url).headers(headers?).send().await.unwrap())
    }
    async fn post(&self, url: &str, query: Query) -> Result<reqwest::Response> {
        let headers = self.generate_header("POST", query, &format!("{}{}", self.base_url, url));
        Ok(self.client.get(url).headers(headers?).send().await.unwrap())
    }
}

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
impl OauthClient for Oauth2Client {
    async fn get(&self, url: &str, query: Query) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, url);
        let url = url::Url::parse_with_params(&url, query)?;

        let headers = self.generate_header();

        dbg!(&url);
        let res = self.client.get(url).headers(headers?).send().await?;
        dbg!(&res);
        Ok(res)
    }
    async fn post(&self, url: &str, query: Query) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, url);
        let headers = self.generate_header();
        Ok(self.client.get(url).headers(headers?).send().await.unwrap())
    }
}
