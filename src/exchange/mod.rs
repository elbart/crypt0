pub(crate) mod kraken_futures;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Exchange {
    KrakenFutures,
}
