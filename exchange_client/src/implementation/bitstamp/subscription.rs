use serde::Serialize;

#[derive(Serialize)]
pub struct Subscription {
    event: String,
    data: Data,
}

#[derive(Serialize)]
struct Data {
    channel: String,
}

impl Subscription {
    pub fn new(symbol: &String) -> Self {
        Subscription {
            event: "bts:subscribe".into(),
            data: Data {
                channel: format!("order_book_{}", symbol),
            },
        }
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
