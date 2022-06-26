/// Structure containing information required for Oauth1.0A authentication
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
