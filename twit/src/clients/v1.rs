use crate::auth::{
    oauth1a::{client::Oauth1aClient, Oauth1aInfo},
    oauth2::{client::Oauth2Client, Oauth2Info},
    AuthClient,
};

mod search;

pub struct V1Client {
    client: Box<dyn AuthClient>,
}

impl V1Client {
    const BASE_URL: &'static str = "https://api.twitter.com/1.1";
    pub fn new(credentials: Oauth1aInfo) -> V1Client {
        V1Client {
            client: Box::new(Oauth1aClient::new(credentials, Self::BASE_URL)),
        }
    }
    pub fn new_apponly(credentials: Oauth2Info) -> V1Client {
        V1Client {
            client: Box::new(Oauth2Client::new(credentials, Self::BASE_URL)),
        }
    }
}
