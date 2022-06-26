use anyhow::Result;
use tabled::object::{Row, Rows};
use tabled::{MaxWidth, Modify, Table, Tabled};
use twit::auth::oauth2::Oauth2Info;
use twit::clients::http::{Oauth2Client, OauthClient};
use twit::clients::v1;

#[derive(Tabled)]
struct TweetTable {
    user: String,
    content: String,
    favo: u64,
    rt: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    load_dotenv::load_dotenv!();
    let i = Oauth2Info::new(env!("V1_BEARER_TOKEN"));
    let client = v1::V1Client::new_apponly(i);
    let res = client.search_tweets("あいうえお").await?;

    let tables = res.statuses.iter().map(|status| {
        //
        TweetTable {
            user: "".to_string(),
            content: status.text.clone(),
            favo: status.favorite_count,
            rt: status.retweet_count,
        }
    });

    let table = Table::new(tables)
        .with(Modify::new(Rows::new(1..)).with(MaxWidth::truncating(10).suffix("...")));

    println!("{table}");

    Ok(())
}
