pub(crate) mod client;
pub(super) mod utils;

/// Structure containing information required for Oauth1.0A authentication
#[derive(Debug)]
pub struct Oauth1aInfo {
    pub(crate) consumer_key: String,
    pub(crate) consumer_secret: String,
    pub(crate) token: String,
    pub(crate) token_secret: String,
}

impl Oauth1aInfo {
    pub fn new(
        consumer_key: &str,
        consumer_secret: &str,
        token: &str,
        token_secret: &str,
    ) -> Oauth1aInfo {
        Oauth1aInfo {
            consumer_key: consumer_key.to_string(),
            consumer_secret: consumer_secret.to_string(),
            token: token.to_string(),
            token_secret: token_secret.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use load_dotenv::load_dotenv;

    load_dotenv!();

    #[tokio::test]
    async fn search_oauth() {
        let auth = crate::auth::oauth1a::Oauth1aInfo::new(
            env!("OAUTH1A_KEY"),
            env!("OAUTH1A_SECRET"),
            env!("OAUTH1A_TOKEN"),
            env!("OAUTH1A_TOKEN_SECRET"),
        );
        let client = crate::clients::v1::V1Client::new(auth);
        let res = client.search_tweets("#rust").await.unwrap();
        dbg!(&res);
    }
}
