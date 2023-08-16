//! This example shows a way of using conditional compilation to enable using
//! [`embassy_mock::MockTimer`] in tests and [`embassy_time::Timer`] in production code without
//! making everything generic over the [`embassy_mock::Timer`] trait.

#![no_std]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::Duration;

#[cfg(not(test))]
use embassy_time::Timer;

#[cfg(test)]
use embassy_mock::time::{MockTimer as Timer, Timer as _};

pub async fn use_a_timer() {
    Timer::after(Duration::from_secs(1)).await;
    // Do something.
    Timer::after(Duration::from_millis(500)).await;
    // Do something else.
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    use_a_timer().await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use embassy_futures::block_on;

    #[test]
    fn use_mocked_timer_type() {
        block_on(use_a_timer());
    }
}
