//! Traits and mocked types to allow unit testing functions that require an
//! [`embassy_time::Ticker`].
//!
//! # Examples
//! ```
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
//!         let mut ticker = MockTicker::expect(1);
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
use embassy_time::{Duration, Ticker as EmbassyTicker};
use snafu::prelude::*;

/// The trait to replace the [`embassy_time::Ticker`] in code to allow the [`MockTicker`] to
/// be used in its place for tests.
pub trait Ticker {
    /// Wrapper for ['embassy_time::Ticker::every()`].
    fn every(duration: Duration) -> Self;

    /// Wrapper for [`embassy_time::Ticker::next()`].
    fn next(&mut self) -> impl Future<Output = ()> + '_;
}

impl Ticker for EmbassyTicker {
    /// Creates a new ticker that ticks at the specified duration interval.
    fn every(duration: Duration) -> Self {
        Self::every(duration)
    }

    /// Waits for the next tick
    fn next(&mut self) -> impl Future<Output = ()> + '_ {
        self.next()
    }
}

/// The errors that are reported by [`MockTicker`].
#[derive(Debug, Snafu, PartialEq)]
pub enum MockTickerError {
    /// The [`MockTicker::next()`] method was called the wrong number of times.
    #[snafu(display("expected to call next {expected} time(s), actually called {actual}"))]
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
/// let mut ticker = MockTicker::expect(3);
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
/// let mut ticker = MockTicker::expect(1); // Expects `next()` to be called once.
/// block_on(ticker.next()); // `next()` is called once.
///
/// // `ticker` is dropped but doesn't panic.
/// ```
///
/// ```should_panic
/// use embassy_futures::block_on;
/// use embassy_mock::time::{MockTicker, Ticker};
///
/// let mut ticker = MockTicker::expect(2); // Expects `next()` to be called twice.
/// block_on(ticker.next()); // `next()` is called only once.
///
/// // `ticker` is dropped and will panic.
/// ```
#[derive(Debug)]
pub struct MockTicker {
    /// The number of expected calls to [`Self::next()`].
    expected: usize,

    /// The number of times [`Self::next()`] has been called.
    times_called: usize,

    /// Has this mock been checked with a call to [`Self::done()`].
    /// If true it is not checked when dropped.
    is_done: bool,
}

impl MockTicker {
    /// Create a [`MockTicker`], providing the expected number of calls to [`Self::next()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use embassy_mock::time::MockTicker;
    ///
    /// # const X: usize = 0;
    /// let ticker = MockTicker::expect(X); // Where `X` is the number of times `next()` should be called
    /// ```
    pub const fn expect(expected: usize) -> Self {
        Self {
            expected,
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
    /// let mut ticker = MockTicker::expect(1);
    /// block_on(ticker.next());
    ///
    /// ticker.done().unwrap();
    /// ```
    ///
    /// ```
    /// use embassy_futures::block_on;
    /// use embassy_mock::time::{MockTicker, MockTickerError, Ticker};
    ///
    /// let mut ticker = MockTicker::expect(4);
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
        let res = if self.times_called != self.expected {
            Err(MockTickerError::WrongNumberOfTicks {
                expected: self.expected,
                actual: self.times_called,
            })
        } else {
            Ok(())
        };

        self.is_done = true;
        res
    }
}

impl Drop for MockTicker {
    /// If [`Self::done()`] has not been called before being dropped then check that the number of
    /// times [`Self::next()`] was called is as expected.
    fn drop(&mut self) {
        if !self.is_done {
            assert_eq!(
                self.expected, self.times_called,
                "expected to call next {} time(s), actually called {}",
                self.expected, self.times_called
            );
        }
    }
}

impl Ticker for MockTicker {
    /// Create a [`MockTicker`] that doesn't require [`Self::done()`] to be called.
    /// This allows a [`MockTicker`] to be created in production code instead of in the test.
    ///
    /// # Examples
    /// ```
    /// use embassy_mock::time::Ticker;
    /// use embassy_time::Duration;
    ///
    /// async fn production_code<T: Ticker>() {
    ///     let mut ticker = T::every(Duration::from_secs(1));
    ///     // Do something...
    ///     ticker.next().await;
    ///     // Do something else...
    /// }
    ///
    /// # test_creating_ticker();
    /// // The unit tests that use the `MockTicker`
    /// #[cfg(test)]
    /// mod tests {
    ///     use super::*;
    /// # }
    ///     use embassy_futures::block_on;
    ///     use embassy_mock::time::MockTicker;
    ///
    ///     #[test]
    ///     # fn hidden_fake_test(){}
    ///     fn test_creating_ticker() {
    ///         // Can't set expectations but at least it is testable
    ///         block_on(production_code::<MockTicker>());
    ///     }
    /// # mod closing {
    /// }
    /// ```
    fn every(_duration: Duration) -> Self {
        Self {
            expected: 0,
            times_called: 0,
            is_done: true, // Mark as done so it won't be checked.
        }
    }

    /// Increment an internal counter of how many times this method is called and return [`Poll::Ready`].
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
        let mut ticker = MockTicker::expect(1);

        block_on(ticker.next());
    }

    #[test]
    fn can_tick_multiple_times_just_drop() {
        let mut ticker = MockTicker::expect(3);

        block_on(ticker.next());
        block_on(ticker.next());
        block_on(ticker.next());
    }

    #[test]
    #[should_panic(expected = "expected to call next 1 time(s), actually called 3")]
    fn tick_too_many_times_just_drop() {
        let mut ticker = MockTicker::expect(1);
        block_on(ticker.next());
        block_on(ticker.next());
        block_on(ticker.next());
    }

    #[test]
    #[should_panic(expected = "expected to call next 3 time(s), actually called 1")]
    fn tick_too_few_times_just_drop() {
        let mut ticker = MockTicker::expect(3);
        block_on(ticker.next());
    }

    #[test]
    fn done_returns_ok() {
        let mut ticker = MockTicker::expect(1);
        block_on(ticker.next());

        let res = ticker.done();

        assert_eq!(res, Ok(()));
    }

    #[test]
    fn done_returns_err_does_not_panic_on_drop() {
        let mut ticker = MockTicker::expect(3);
        block_on(ticker.next());

        let res = ticker.done();

        let expected = Err(MockTickerError::WrongNumberOfTicks {
            expected: 3,
            actual: 1,
        });
        assert_eq!(res, expected);
    }
}
