use std::pin::Pin;

use async_trait::async_trait;
use async_tungstenite::{
    tokio::{connect_async, TokioAdapter},
    tungstenite::Message,
    WebSocketStream,
};
use futures::prelude::*;
type Socket = WebSocketStream<
    async_tungstenite::stream::Stream<
        TokioAdapter<tokio::net::TcpStream>,
        TokioAdapter<Pin<Box<tokio_openssl::SslStream<tokio::net::TcpStream>>>>,
    >,
>;

#[async_trait]
pub trait WSClient {
    async fn connect(&mut self, uri: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn read_forever(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn subscribe(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct KrakenFuturesWSClient {
    socket: Option<Socket>,
}

#[async_trait]
impl WSClient for KrakenFuturesWSClient {
    async fn connect(&mut self, uri: String) -> Result<(), Box<dyn std::error::Error>> {
        let (socket, _) = connect_async(uri).await?;
        self.socket = Some(socket);

        Ok(())
    }

    async fn subscribe(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let msg = Message::Text(
            r#"{"event":"subscribe", "feed":"book", "product_ids":["PI_XBTUSD"]}"#.into(),
        );
        self.socket.as_mut().unwrap().send(msg).await?;
        Ok(())
    }

    async fn read_forever(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(msg) = self.socket.as_mut().unwrap().next().await {
            let msg = msg?;

            if msg.is_binary() {
                continue;
            }

            println!("{}", msg);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_kraken_futures_baseline() {
        let mut c = KrakenFuturesWSClient { socket: None };
        c.connect("wss://futures.kraken.com/ws/v1".into())
            .await
            .unwrap();

        c.subscribe().await.unwrap();
        c.read_forever().await.unwrap();
    }
}
