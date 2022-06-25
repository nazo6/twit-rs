use serde::Deserialize;

pub struct Query {
    a: String,
}

#[derive(Deserialize)]
pub struct Response {
    a: String,
}
