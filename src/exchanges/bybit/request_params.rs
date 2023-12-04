use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct RequestParams<'a> {
    op: &'a str,
    args: Option<Vec<String>>,
}

impl RequestParams<'_> {
    pub fn subscribe(topics: &Vec<String>) -> Self {
        RequestParams {
            op: "subscribe",
            args: Some(topics.clone()),
        }
    }

    pub fn ping() -> Self {
        RequestParams {
            op: "ping",
            args: None,
        }
    }
}
