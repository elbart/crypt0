pub mod bitmex;
pub mod bybit;
pub mod ftx;
pub mod kraken_futures;
pub(crate) mod util;
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

pub struct WSClient {
    socket: Socket,
}

impl WSClient {
    pub async fn connect(uri: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            socket: connect_async(uri).await?.0,
        })
    }

    pub async fn send(&mut self, message: Message) -> Result<(), Box<dyn std::error::Error>> {
        self.socket.send(message).await?;
        Ok(())
    }

    pub async fn read_forever(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(msg) = self.socket.next().await {
            let msg = msg?;

            println!("{}", msg);
            if msg.is_binary() {
                continue;
            }
        }
        Ok(())
    }
}

pub trait CryptoMessage: Sync + Send {
    fn get_message(&self) -> Message;
}

#[async_trait]
pub trait WSCrypto {
    async fn subscribe(&mut self, message: Message) -> Result<(), Box<dyn std::error::Error>>;
}
