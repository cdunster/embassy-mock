//! This example shows a way of using the mocked types without making all functions and types
//! generic over the provided traits. It does this via conditional compilation.

#![no_std]
#![feature(type_alias_impl_trait)]

use embassy_time::Duration;

#[cfg(not(test))]
use {
    embassy_executor::Spawner,
    embassy_time::{Ticker, Timer},
};

#[cfg(test)]
use embassy_mock::{
    executor::{MockSpawner as Spawner, Spawner as _},
    time::{MockTicker as Ticker, MockTimer as Timer, Ticker as _, Timer as _},
};

#[embassy_executor::task]
async fn task_with_timer() -> ! {
    let mut val = 0;
    loop {
        adding_with_timer(&mut val, Duration::from_secs(1)).await;
    }
}

async fn adding_with_timer(val: &mut u64, delay: Duration) {
    *val = val.wrapping_add(1);
    Timer::after(delay).await;
}

#[embassy_executor::task]
async fn task_with_ticker() {
    let mut ticker = Ticker::every(Duration::from_secs(1));
    let mut val = 0;
    loop {
        adding_with_ticker(&mut val, &mut ticker).await;
    }
}

async fn adding_with_ticker(val: &mut u64, ticker: &mut Ticker) {
    *val = val.wrapping_add(1);
    ticker.next().await;
}

pub fn spawn_tasks(spawner: &Spawner) {
    spawner.spawn(task_with_timer()).unwrap();
    spawner.spawn(task_with_ticker()).unwrap();
}

#[cfg(not(test))]
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    spawn_tasks(&spawner);
}

#[cfg(test)]
mod tests {
    use super::*;
    use embassy_futures::block_on;
    use embassy_mock::{executor::MockSpawner, time::MockTicker};

    #[test]
    fn mocking_timer() {
        let mut val = 0;
        block_on(adding_with_timer(&mut val, Duration::from_secs(1)));

        assert_eq!(val, 1);
    }

    #[test]
    fn mocking_ticker() {
        let mut val = 0;
        let mut ticker = MockTicker::expect(1);
        block_on(adding_with_ticker(&mut val, &mut ticker));

        assert_eq!(val, 1);

        ticker.done().unwrap();
    }

    #[test]
    fn mocking_spawner() {
        let spawner = MockSpawner::expect(2);
        spawn_tasks(&spawner);

        spawner.done().unwrap();
    }
}
