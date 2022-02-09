use async_trait::async_trait;
use async_tungstenite::tungstenite::Message;

use super::{util::baseline_ws_client, WSClient, WSCrypto};

const WS_URL: &str = "wss://stream.bybit.com/realtime";

pub struct BybitWSClient {
    inner: WSClient,
}

baseline_ws_client!(BybitWSClient);

pub enum BybitMessage {
    Subscribe {
        feed: String,
        product_ids: Vec<String>,
    },
}

impl From<BybitMessage> for Message {
    fn from(msg: BybitMessage) -> Message {
        match msg {
            BybitMessage::Subscribe { feed, product_ids } => {
                let topics = serde_json::to_string(
                    &product_ids
                        .iter()
                        .map(|product_id| format!("{}.{}", feed, product_id))
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
    async fn test_bybit_ws_base_line() {
        let mut c = BybitWSClient::new().await.unwrap();

        c.subscribe(
            BybitMessage::Subscribe {
                feed: "orderBookL2_25".into(),
                product_ids: vec!["BTCUSD".into()],
            }
            .into(),
        )
        .await
        .unwrap();
        c.inner.read_forever().await.unwrap();
    }
}
