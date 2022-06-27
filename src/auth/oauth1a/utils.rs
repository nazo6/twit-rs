use anyhow::Result;
use base64::encode as base64_encode;
use reqwest::header::HeaderMap;
use urlencoding::encode;

use crate::Query;

pub fn generate_sig(
    url: &str,
    query: Query,
    method: &str,
    consumer_secret: &str,
    token_secret: &str,
) -> String {
    let key = format!("{}&{}", encode(consumer_secret), encode(token_secret));
    let mut query = query;
    query.sort_by(|a, b| a.0.cmp(&b.0));
    let query = query
        .iter()
        .map(|q| format!("{}={}", encode(&q.0), encode(&q.1)))
        .collect::<Vec<String>>()
        .join("&");
    let query = encode(&query);
    let method = encode(method);
    let url = encode(url);
    let data = format!("{method}&{url}&{query}");

    dbg!(&data);

    let hash = hmacsha1::hmac_sha1(key.as_bytes(), data.as_bytes());
    base64_encode(&hash)
}

pub fn generate_header(
    method: &str,
    query: &Query,
    url: &str,
    consumer_key: &str,
    consumer_secret: &str,
    token: &str,
    token_secret: &str,
) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    let timestamp = chrono::Utc::now().timestamp().to_string();

    let mut oauth_parameters: Vec<(String, String)> = vec![
        ("oauth_consumer_key".to_string(), consumer_key.to_string()),
        ("oauth_nonce".to_string(), timestamp.to_string()),
        ("oauth_nonce".to_string(), timestamp.to_string()),
        (
            "oauth_signature_method".to_string(),
            "HMAC-SHA1".to_string(),
        ),
        ("oauth_timestamp".to_string(), timestamp),
        ("oauth_version".to_string(), "1.0".to_string()),
    ];
    if !token.is_empty() {
        oauth_parameters.push(("oauth_token".to_string(), token.to_string()));
    }

    let mut sig_query = query.clone();
    sig_query.append(&mut oauth_parameters.clone());
    let oauth_signature = generate_sig(url, sig_query, method, consumer_secret, token_secret);
    oauth_parameters.push((
        "oauth_signature".to_string(),
        encode(&oauth_signature).to_string(),
    ));

    let auth_str = oauth_parameters
        .iter()
        .map(|p| format!("{}=\"{}\"", p.0, p.1))
        .collect::<Vec<String>>()
        .join(", ");
    let auth_str = format!("OAuth {}", auth_str);

    headers.append("Authorization", auth_str.parse()?);

    Ok(headers)
}

#[cfg(test)]
mod tests {
    const CONSUMER_KEY: &str = "abcde";
    const CONSUMER_SECRET: &str = "dghijk";
    const TOKEN: &str = "a-b-c";
    const TOKEN_SECRET: &str = "ABC";
    const TIMESTAMP: i64 = 1656310925;
    const QUERY: &str = "#rust";
    #[test]
    fn generate_oauth1a_sig() {
        let url = "https://api.twitter.com/1.1/search/tweets.json";
        let query: Vec<(String, String)> = vec![
            ("oauth_consumer_key".to_string(), CONSUMER_KEY.to_string()),
            // ("oauth_nonce".to_string(), timestamp.to_string()),
            ("oauth_nonce".to_string(), TIMESTAMP.to_string()),
            (
                "oauth_signature_method".to_string(),
                "HMAC-SHA1".to_string(),
            ),
            // ("oauth_timestamp".to_string(), timestamp),
            ("oauth_timestamp".to_string(), TIMESTAMP.to_string()),
            ("oauth_version".to_string(), "1.0".to_string()),
            ("oauth_token".to_string(), TOKEN.to_string()),
            ("q".to_string(), QUERY.to_string()),
        ];
        let sig = crate::auth::oauth1a::utils::generate_sig(
            url,
            query,
            "GET",
            CONSUMER_SECRET,
            TOKEN_SECRET,
        );
        assert_eq!("oN1/ghZ/MEOsJ4stbAxqosyFO/c=", sig)
    }
}
