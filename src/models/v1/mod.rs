use serde::Deserialize;

pub mod search;

#[derive(Deserialize, Debug, Clone)]
pub struct Tweet {
    pub created_at: String,
    pub id_str: String,
    pub text: String,
    pub full_text: Option<String>,
    pub source: String,
    pub truncated: bool,
    pub in_reply_to_status_id_str: Option<String>,
    pub in_reply_to_user_id_str: Option<String>,
    pub in_reply_to_screen_name: Option<String>,
    pub user: User,
    pub coordinates: Option<Coordinate>,
    pub place: Option<Place>,
    pub quoted_status_id_str: Option<String>,
    pub is_quote_status: bool,
    pub quoted_status: Option<Box<Tweet>>,
    pub retweeted_status: Option<Box<Tweet>>,
    pub quote_count: Option<u64>,
    pub reply_count: Option<u64>,
    pub retweet_count: u64,
    pub favorite_count: u64,
    pub entities: TweetEntities,
    pub extended_entities: Option<TweetExtendedEntities>,
    pub favorited: Option<bool>,
    pub retweeted: bool,
    pub possibly_sensitive: Option<bool>,
    pub filter_level: Option<TweetFilterLevel>,
    pub lang: String,
    pub display_text_range: Option<(u64, u64)>,
    pub current_user_retweet: Option<TweetCurrentUserRetweet>,
    pub withheld_copyright: Option<bool>,
    pub withheld_in_countries: Option<Vec<String>>,
    pub withheld_scope: Option<String>,
    pub card_uri: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub enum TweetFilterLevel {
    #[serde(rename = "none")]
    No,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

#[derive(Deserialize, Clone, Debug)]
pub struct User {}

#[derive(Deserialize, Clone, Debug)]
pub struct TweetCurrentUserRetweet {
    pub id: u64,
    pub id_str: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Coordinate {}
#[derive(Deserialize, Clone, Debug)]
pub struct Place {}
#[derive(Deserialize, Clone, Debug)]
pub struct TweetEntities {}
#[derive(Deserialize, Clone, Debug)]
pub struct TweetExtendedEntities {}
