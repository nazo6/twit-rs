use anyhow::Result;
use serde::Deserialize;

pub struct Oauth2Info {
    pub(crate) bearer: String,
    pub(crate) refresh: Option<String>,
}

impl Oauth2Info {
    const API_BASE: &'static str = "https://api.twitter.com";
    /// Create new Oauth2 credential info from bearer token.
    pub fn new(bearer: &str) -> Oauth2Info {
        Oauth2Info {
            bearer: bearer.to_string(),
            refresh: None,
        }
    }
    /// Crate new Oauth2 credential info using consumer key and secret through app-only auth.
    pub async fn new_apponly(key: &str, secret: &str) -> Result<Self> {
        let bearer_credentials = base64::encode(format!("{}:{}", key, secret));
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}{}", Self::API_BASE, "/oauth2/token"))
            .header("authorization", bearer_credentials)
            .header("content-type", "application/x-www-form-urlencoded")
            .body("grant_type=client_credentials")
            .send()
            .await?
            .json::<TwitterOauth2TokenResponse>()
            .await?;
        if res.token_type == "bearer" {
            Ok(Self {
                bearer: res.access_token,
                refresh: None,
            })
        } else {
            Err(anyhow::anyhow!("No bearer token found"))
        }
    }
}

#[derive(Deserialize)]
struct TwitterOauth2TokenResponse {
    token_type: String,
    access_token: String,
}
