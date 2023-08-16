//! A mocked version of the `embassy-time` crate.

pub mod ticker;
pub mod timer;

pub use ticker::{MockTicker, MockTickerError, Ticker};
pub use timer::{MockTimer, Timer};
