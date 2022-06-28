use proc_macro2::TokenStream;
use syn::{
    parse::{ParseStream, Parser},
    Error, Result,
};

#[proc_macro]
pub fn api(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    api_impl(input.into()).into()
}

fn api_impl(tokens: TokenStream) -> TokenStream {
    api_parse
        .parse2(tokens)
        .unwrap_or_else(Error::into_compile_error)
}

fn api_parse(input: ParseStream) -> Result<TokenStream> {
    let p = input.parse()?;
}

#[cfg(test)]
mod tests {
    use crate::api_impl;
    use quote::quote;

    struct RequestStructTest {
        q: String,
        until: String,
    }
    struct ResponseStructTest {
        status: String,
    }

    #[test]
    fn test_api() {
        assert_eq!(
            api_impl(quote![
                "search_tweets"
                "GET",
                "/search/tweets.json",
                RequestStructTest,
                ResponseStructTest
            ])
            .to_string(),
            quote!(
                pub async fn search_tweets(&self, q: &str) -> Result<SearchTweetsResponse> {
                    let v: Vec<(String, String)> = vec![("q".to_string(), q.to_string())];
                    let res: SearchTweetsResponse = self
                        .client
                        .get("/search/tweets.json", v)
                        .await?
                        .json()
                        .await?;
                    Ok(res)
                }
            )
            .to_string()
        );
    }
}
