#![no_std]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_mock::time::Ticker;

pub async fn use_a_ticker<T: Ticker>(ticker: &mut T) {
    ticker.next().await;
    // Do something.
    ticker.next().await;
    // Do something else.
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {}

#[cfg(test)]
mod tests {
    use super::*;
    use embassy_futures::block_on;
    use embassy_mock::time::MockTicker;

    #[test]
    fn use_mocked_ticker_type() {
        let mut ticker = MockTicker::expect(2);
        block_on(ticker.next());
        block_on(ticker.next());

        ticker.done().unwrap();
    }
}
