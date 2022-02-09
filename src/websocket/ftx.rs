use async_trait::async_trait;
#[cfg(feature = "tokio-openssl")]
use async_tungstenite_tokio_openssl::tungstenite::Message;

use super::{util::baseline_ws_client, WSClient, WSCrypto};

const WS_URL: &str = "wss://ftx.com/ws/";

pub struct FtxWSClient {
    inner: WSClient,
}

baseline_ws_client!(FtxWSClient);

pub enum FtxMessage {
    Subscribe { feed: String, product_id: String },
}

impl From<FtxMessage> for Message {
    fn from(msg: FtxMessage) -> Self {
        match msg {
            FtxMessage::Subscribe { feed, product_id } => {
                let text = format!(
                    r#"{{"op":"subscribe", "channel": "{}", "market": "{}"}}"#,
                    feed, product_id
                );
                Message::Text(text)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_ftx_ws_base_line() {
        let mut c = FtxWSClient::new().await.unwrap();

        c.subscribe(
            FtxMessage::Subscribe {
                feed: "orderbook".into(),
                product_id: "BTC-PERP".into(),
            }
            .into(),
        )
        .await
        .unwrap();
        c.inner.read_forever().await.unwrap();
    }
}
