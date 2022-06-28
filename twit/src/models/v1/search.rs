use serde::Deserialize;

use super::Tweet;

#[derive(Deserialize, Clone, Debug)]
pub struct SearchTweetsResponse {
    pub statuses: Vec<Tweet>,
}
