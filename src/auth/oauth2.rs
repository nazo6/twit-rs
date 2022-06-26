pub struct Oauth2Info {
    pub(crate) bearer: String,
}

impl Oauth2Info {
    pub fn new(bearer: &str) -> Oauth2Info {
        Oauth2Info {
            bearer: bearer.to_string(),
        }
    }
}
