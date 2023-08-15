//! A mocked version of the `embassy-time` crate.

pub mod ticker;

pub use ticker::{MockTicker, MockTickerError, Ticker};
