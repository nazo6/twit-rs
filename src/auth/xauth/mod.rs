use crate::auth::oauth1a::utils::generate_header;
use anyhow::Result;

mod client;

/// Structure containing information required for XAuth
#[derive(Debug)]
pub struct XauthInfo {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub token: String,
    pub token_secret: String,
}

impl XauthInfo {
    pub fn new(
        consumer_key: &str,
        consumer_secret: &str,
        token: &str,
        token_secret: &str,
    ) -> XauthInfo {
        XauthInfo {
            consumer_key: consumer_key.to_string(),
            consumer_secret: consumer_secret.to_string(),
            token: token.to_string(),
            token_secret: token_secret.to_string(),
        }
    }
    pub async fn from_pass(
        consumer_key: &str,
        consumer_secret: &str,
        username: &str,
        password: &str,
    ) -> Result<XauthInfo> {
        let url = "https://api.twitter.com/oauth/access_token";
        let query = vec![
            ("x_auth_mode".to_string(), "client_auth".to_string()),
            ("x_auth_username".to_string(), username.to_string()),
            ("x_auth_password".to_string(), password.to_string()),
        ];
        let headers = generate_header("POST", &query, url, consumer_key, consumer_secret, "", "")?;
        let res = reqwest::Client::new()
            .post(url)
            .query(&query)
            .headers(headers)
            .send()
            .await?;
        dbg!(&res.status());
        let body = res.text().await?;
        dbg!(&body);
        let mut parts = body.split('&');
        let token = parts.next().unwrap().split('=').nth(1).unwrap();
        let token_secret = parts.next().unwrap().split('=').nth(1).unwrap();

        Ok(XauthInfo::new(
            consumer_key,
            consumer_secret,
            token,
            token_secret,
        ))
    }
}

#[cfg(test)]
mod tests {
    use load_dotenv::load_dotenv;

    load_dotenv!();

    #[tokio::test]
    async fn auth_by_xauth() {
        let client = crate::auth::xauth::XauthInfo::from_pass(
            env!("XAUTH_KEY"),
            env!("XAUTH_SECRET"),
            env!("XAUTH_USERNAME"),
            env!("XAUTH_PASSWORD"),
        )
        .await;
        dbg!(client);
    }
}
