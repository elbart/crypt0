macro_rules! baseline_ws_client {
    ($client:ty) => {
        impl $client {
            pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
                Ok(Self {
                    inner: WSClient::connect(WS_URL).await?,
                })
            }

            pub async fn read_forever(&mut self) -> Result<(), Box<dyn std::error::Error>> {
                self.inner.read_forever().await
            }
        }

        #[async_trait]
        impl WSCrypto for $client {
            async fn subscribe(
                &mut self,
                message: Message,
            ) -> Result<(), Box<dyn std::error::Error>> {
                self.inner.send(message.into()).await
            }
        }
    };
}

pub(crate) use baseline_ws_client;
