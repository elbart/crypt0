use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Exchange {
    Bybit { feed: String },
    Bitmex { feed: String },
    Ftx { feed: String },
    KrakenFutures { feed: String },
}
