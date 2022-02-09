use async_trait::async_trait;
#[cfg(feature = "tokio-openssl")]
use async_tungstenite_tokio_openssl::tungstenite::Message;

use super::{util::baseline_ws_client, WSClient, WSCrypto};

const WS_URL: &str = "wss://futures.kraken.com/ws/v1";

pub struct KrakenFuturesWSClient {
    inner: WSClient,
}

baseline_ws_client!(KrakenFuturesWSClient);

pub enum KrakenFuturesMessage {
    Subscribe {
        feed: String,
        product_ids: Vec<String>,
    },
}

impl From<KrakenFuturesMessage> for Message {
    fn from(msg: KrakenFuturesMessage) -> Self {
        match msg {
            KrakenFuturesMessage::Subscribe { feed, product_ids } => {
                let text = format!(
                    r#"{{"event":"subscribe", "feed":"{}", "product_ids":{}}}"#,
                    feed,
                    serde_json::to_string(&product_ids).unwrap()
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
    async fn test_kraken_futures_ws_base_line() {
        let mut c = KrakenFuturesWSClient::new().await.unwrap();

        c.subscribe(
            KrakenFuturesMessage::Subscribe {
                feed: "book".into(),
                product_ids: vec!["PI_XBTUSD".into()],
            }
            .into(),
        )
        .await
        .unwrap();
        c.inner.read_forever().await.unwrap();
    }
}
