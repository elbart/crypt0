use crate::{
    exchange::Exchange,
    websocket::{
        bitmex::{BitmexMessage, BitmexWSClient},
        bybit::{BybitMessage, BybitWSClient},
        ftx::{FtxMessage, FtxWSClient},
        kraken_futures::{KrakenFuturesMessage, KrakenFuturesWSClient},
        WSCrypto,
    },
};

pub struct Crawler {
    exchange: Exchange,
    product_ids: Vec<String>,
}

impl Crawler {
    pub fn new(exchange: Exchange, product_ids: Vec<String>) -> Self {
        Self {
            exchange,
            product_ids,
        }
    }

    pub async fn crawl(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.exchange {
            Exchange::Bitmex { feed } => {
                let mut ws_client = BitmexWSClient::new().await?;
                ws_client
                    .subscribe(
                        BitmexMessage::Subscribe {
                            feed: feed.clone(),
                            product_ids: self.product_ids.clone(),
                        }
                        .into(),
                    )
                    .await?;
                ws_client.read_forever().await?;
            }
            Exchange::KrakenFutures { feed } => {
                let mut ws_client = KrakenFuturesWSClient::new().await?;
                ws_client
                    .subscribe(
                        KrakenFuturesMessage::Subscribe {
                            feed: feed.clone(),
                            product_ids: self.product_ids.clone(),
                        }
                        .into(),
                    )
                    .await?;
                ws_client.read_forever().await?;
            }
            Exchange::Bybit { feed } => {
                let mut ws_client = BybitWSClient::new().await?;
                ws_client
                    .subscribe(
                        BybitMessage::Subscribe {
                            feed: feed.clone(),
                            product_ids: self.product_ids.clone(),
                        }
                        .into(),
                    )
                    .await?;
                ws_client.read_forever().await?;
            }
            Exchange::Ftx { feed } => {
                let mut ws_client = FtxWSClient::new().await?;
                ws_client
                    .subscribe(
                        FtxMessage::Subscribe {
                            feed: feed.clone(),
                            product_id: self.product_ids.get(0).unwrap().into(),
                        }
                        .into(),
                    )
                    .await?;
                ws_client.read_forever().await?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Crawler;
    use crate::exchange::Exchange;

    #[tokio::test]
    async fn crawler_baseline() {
        let mut crawler = Crawler::new(
            Exchange::Bitmex {
                feed: "orderBookL2_25".into(),
            },
            vec!["XBTUSD".into()],
        );
        crawler.crawl().await.unwrap();
    }
}
