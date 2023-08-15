use core::{
    future::{poll_fn, Future},
    task::Poll,
};
use embassy_time::Ticker as EmbassyTicker;
use thiserror_no_std::Error;

pub trait Ticker {
    fn next(&mut self) -> impl Future<Output = ()> + '_;
}

impl Ticker for EmbassyTicker {
    fn next(&mut self) -> impl Future<Output = ()> + '_ {
        self.next()
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum MockTickerError {
    #[error("expected to call next {expected} time(s), actually called {actual}")]
    WrongNumberOfTicks { expected: usize, actual: usize },
}

pub struct MockTicker<const N: usize> {
    times_called: usize,
    is_done: bool,
}

impl<const N: usize> MockTicker<N> {
    pub const fn new() -> Self {
        Self {
            times_called: 0,
            is_done: false,
        }
    }

    pub fn done(mut self) -> Result<(), MockTickerError> {
        let res = if self.times_called != N {
            Err(MockTickerError::WrongNumberOfTicks {
                expected: N,
                actual: self.times_called,
            })
        } else {
            Ok(())
        };

        self.is_done = true;
        res
    }
}

impl<const N: usize> Drop for MockTicker<N> {
    fn drop(&mut self) {
        if !self.is_done {
            assert_eq!(
                N, self.times_called,
                "expected to call next {} time(s), actually called {}",
                N, self.times_called
            );
        }
    }
}

impl<const N: usize> Ticker for MockTicker<N> {
    fn next(&mut self) -> impl Future<Output = ()> + '_ {
        self.times_called = self.times_called.checked_add(1).unwrap();
        poll_fn(|_cx| Poll::Ready(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embassy_futures::block_on;

    #[test]
    fn can_tick_once_just_drop() {
        let mut ticker = MockTicker::<1>::new();

        block_on(ticker.next());
    }

    #[test]
    fn can_tick_multiple_times_just_drop() {
        let mut ticker = MockTicker::<3>::new();

        block_on(ticker.next());
        block_on(ticker.next());
        block_on(ticker.next());
    }

    #[test]
    #[should_panic(expected = "expected to call next 1 time(s), actually called 3")]
    fn tick_too_many_times_just_drop() {
        let mut ticker = MockTicker::<1>::new();
        block_on(ticker.next());
        block_on(ticker.next());
        block_on(ticker.next());
    }

    #[test]
    #[should_panic(expected = "expected to call next 3 time(s), actually called 1")]
    fn tick_too_few_times_just_drop() {
        let mut ticker = MockTicker::<3>::new();
        block_on(ticker.next());
    }

    #[test]
    fn done_returns_ok() {
        let mut ticker = MockTicker::<1>::new();
        block_on(ticker.next());

        let res = ticker.done();

        assert_eq!(res, Ok(()));
    }

    #[test]
    fn done_returns_err_does_not_panic_on_drop() {
        let mut ticker = MockTicker::<3>::new();
        block_on(ticker.next());

        let res = ticker.done();

        let expected = Err(MockTickerError::WrongNumberOfTicks {
            expected: 3,
            actual: 1,
        });
        assert_eq!(res, expected);
    }
}
