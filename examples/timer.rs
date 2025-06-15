#![no_std]

use embassy_executor::Spawner;
use embassy_mock::time::Timer;
use embassy_time::Duration;

pub async fn use_a_timer<T: Timer>() {
    T::after(Duration::from_secs(1)).await;
    // Do something.
    T::after(Duration::from_millis(500)).await;
    // Do something else.
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    use_a_timer::<embassy_time::Timer>().await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use embassy_futures::block_on;
    use embassy_mock::time::MockTimer;

    #[test]
    fn use_mocked_timer_type() {
        block_on(use_a_timer::<MockTimer>());
    }
}
