use async_trait::async_trait;
use async_tungstenite::tungstenite::Message;

use super::{util::baseline_ws_client, WSClient, WSCrypto};

const WS_URL: &str = "wss://ws.bitmex.com/realtime";

pub struct BitmexWSClient {
    inner: WSClient,
}

baseline_ws_client!(BitmexWSClient);

pub enum BitmexMessage {
    Subscribe {
        feed: String,
        product_ids: Vec<String>,
    },
}

impl From<BitmexMessage> for Message {
    fn from(msg: BitmexMessage) -> Self {
        match msg {
            BitmexMessage::Subscribe { feed, product_ids } => {
                let topics = serde_json::to_string(
                    &product_ids
                        .iter()
                        .map(|product_id| format!("{}:{}", feed, product_id))
                        .collect::<Vec<String>>(),
                )
                .unwrap();
                let text = format!(r#"{{"op":"subscribe", "args": {}}}"#, topics,);
                Message::Text(text)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_bitmex_ws_base_line() {
        let mut c = BitmexWSClient::new().await.unwrap();

        c.subscribe(
            BitmexMessage::Subscribe {
                feed: "orderBookL2_25".into(),
                product_ids: vec!["XBTUSD".into()],
            }
            .into(),
        )
        .await
        .unwrap();
        c.inner.read_forever().await.unwrap();
    }
}
