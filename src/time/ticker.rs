//! Traits and mocked types to allow unit testing functions that require an
//! [`embassy_time::Ticker`].
//!
//! # Examples
//! ```
//! # #![feature(type_alias_impl_trait)]
//! #
//! use embassy_mock::time::Ticker;
//! use embassy_time::Duration;
//!
//! // Generic over the `Ticker` trait
//! async fn wait_for_ticker<T: Ticker>(ticker: &mut T) {
//!     ticker.next().await;
//! }
//!
//! // The real task that runs on the Embassy executor.
//! #[embassy_executor::task]
//! async fn some_task() {
//!     let mut ticker = embassy_time::Ticker::every(Duration::from_secs(1));
//!     wait_for_ticker(&mut ticker).await;
//! }
//!
//! # test_ticking();
//! // The unit tests that use the `MockTicker`.
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//! # }
//!     use embassy_futures::block_on;
//!     use embassy_mock::time::MockTicker;
//!
//!     #[test]
//!     # fn hidden_fake_test(){}
//!     fn test_ticking() {
//!         let mut ticker = MockTicker::<1>::new();
//!         block_on(wait_for_ticker(&mut ticker));
//!
//!         ticker.done().unwrap();
//!     }
//! # mod closing {
//! }
//! ```

use core::{
    future::{poll_fn, Future},
    task::Poll,
};
use embassy_time::Ticker as EmbassyTicker;
use thiserror_no_std::Error;

/// The trait to replace the [`embassy_time::Ticker`] in code to allow the [`MockTicker`] to
/// be used in its place for tests.
pub trait Ticker {
    /// Wrapper for [`embassy_time::Ticker::next()`].
    fn next(&mut self) -> impl Future<Output = ()> + '_;
}

impl Ticker for EmbassyTicker {
    /// Waits for the next tick
    fn next(&mut self) -> impl Future<Output = ()> + '_ {
        self.next()
    }
}

/// The errors that are reported by [`MockTicker`].
#[derive(Debug, Error, PartialEq)]
pub enum MockTickerError {
    /// The [`MockTicker::next()`] method was called the wrong number of times.
    #[error("expected to call next {expected} time(s), actually called {actual}")]
    WrongNumberOfTicks {
        /// The expected number of calls to [`MockTicker::next()`].
        expected: usize,

        /// The actual number of times [`MockTicker::next()`] was called.
        actual: usize,
    },
}

/// A mocked version of [`embassy_time::Ticker`] that can be used in its place for unit tests.
///
/// This mocked version counts how many times [`Self::next()`] is called and can be checked if
/// [`Self::next()`] was called the correct number of times using [`Self::done()`]. If
/// [`Self::done()`] is not called then it asserts that [`Self::next()`] was called the correct
/// number of times when dropped which causes a panic if incorrect.
///
/// The const generic argument `N` is used to declare how many times [`Self::next()`] should be
/// called.
///
/// # Panics
///
/// Panics if [`Self::next()`] called the wrong number of times and [`Self`] is dropped before
/// calling [`Self::done()`].
///
/// # Examples
///
/// ```
/// use embassy_futures::block_on;
/// use embassy_mock::time::{MockTicker, MockTickerError, Ticker};
///
/// let mut ticker = MockTicker::<3>::new();
/// block_on(ticker.next());
///
/// let res = ticker.done();
///
/// let expected = Err(MockTickerError::WrongNumberOfTicks {
///     expected: 3,
///     actual: 1,
/// });
/// assert_eq!(res, expected);
/// ```
///
/// ```
/// use embassy_futures::block_on;
/// use embassy_mock::time::{MockTicker, Ticker};
///
/// let mut ticker = MockTicker::<1>::new(); // Expects `next()` to be called once.
/// block_on(ticker.next()); // `next()` is called once.
///
/// // `ticker` is dropped but doesn't panic.
/// ```
///
/// ```should_panic
/// use embassy_futures::block_on;
/// use embassy_mock::time::{MockTicker, Ticker};
///
/// let mut ticker = MockTicker::<2>::new(); // Expects `next()` to be called twice.
/// block_on(ticker.next()); // `next()` is called only once.
///
/// // `ticker` is dropped and will panic.
/// ```
#[derive(Debug)]
pub struct MockTicker<const N: usize> {
    times_called: usize,
    is_done: bool,
}

impl<const N: usize> MockTicker<N> {
    /// Create a new [`MockTicker`].
    ///
    /// # Examples
    ///
    /// ```
    /// use embassy_mock::time::MockTicker;
    ///
    /// # const X: usize = 0;
    /// let ticker = MockTicker::<X>::new(); // Where `X` is the number of times `next()` should be called
    /// ```
    pub const fn new() -> Self {
        Self {
            times_called: 0,
            is_done: false,
        }
    }

    /// Mark the [`MockTicker`] as done and check if [`Self::next()`] was called the correct
    /// number of times.
    ///
    /// This is a cleaner way of testing that [`Self::next()`] is called the correct number of
    /// times as [`MockTicker`] doesn't cause a panic when dropped if this method is called
    /// beforehand, it also returns a [`Result<(), MockTickerError>`] which allows checking
    /// the outcome of the mock.
    ///
    /// # Examples
    ///
    /// ```
    /// use embassy_futures::block_on;
    /// use embassy_mock::time::{MockTicker, Ticker};
    ///
    /// let mut ticker = MockTicker::<1>::new();
    /// block_on(ticker.next());
    ///
    /// ticker.done().unwrap();
    /// ```
    ///
    /// ```
    /// use embassy_futures::block_on;
    /// use embassy_mock::time::{MockTicker, MockTickerError, Ticker};
    ///
    /// let mut ticker = MockTicker::<4>::new();
    /// block_on(ticker.next());
    ///
    /// let res = ticker.done();
    ///
    /// let expected = Err(MockTickerError::WrongNumberOfTicks {
    ///     expected: 4,
    ///     actual: 1,
    /// });
    /// assert_eq!(res, expected);
    ///
    /// // This doesn't panic when `ticker` is dropped as `ticker.done()` was called.
    /// ```
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
