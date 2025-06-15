//! Traits and mocked types to allow unit testing functions that require an
//! [`embassy_time::Timer`].
//!
//! # Examples
//! ```
//! use embassy_mock::time::Timer;
//! use embassy_time::Duration;
//!
//! // Generic over the `Timer` trait
//! async fn wait_for_timer<T: Timer>() {
//!     T::after(Duration::from_secs(1)).await;
//!     // Do something..
//! }
//!
//! // The real task that runs on the Embassy executor.
//! #[embassy_executor::task]
//! async fn some_task() {
//!     wait_for_timer::<embassy_time::Timer>().await;
//! }
//!
//! # test_timer_after();
//! // The unit tests that use the `MockTimer`.
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//! # }
//!     use embassy_futures::block_on;
//!     use embassy_mock::time::MockTimer;
//!
//!     #[test]
//!     # fn hidden_fake_test(){}
//!     fn test_timer_after() {
//!         block_on(wait_for_timer::<MockTimer>());
//!     }
//! # mod closing {
//! }
//! ```

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use embassy_time::{Duration, Timer as EmbassyTimer};

/// The trait to replace the [`embassy_time::Timer`] in code to allow the [`MockTimer`] to
/// be used in its place for tests.
pub trait Timer: Future {
    /// Wrapper for [`embassy_time::Timer::after()`].
    fn after(duration: Duration) -> Self;
}

impl Timer for EmbassyTimer {
    /// Expire after specified [`Duration`].
    /// This can be used as a sleep abstraction.
    ///
    /// Example:
    /// ``` no_run
    /// # fn foo() {}
    /// use embassy_time::{Duration, Timer};
    ///
    /// #[embassy_executor::task]
    /// async fn demo_sleep_seconds() {
    ///     // suspend this task for one second.
    ///     Timer::after(Duration::from_secs(1)).await;
    /// }
    /// ```
    fn after(duration: Duration) -> Self {
        Self::after(duration)
    }
}

/// A mocked version of [`embassy_time::Timer`] that can be used in its place for unit tests.
///
/// This mocked version just immediately returns [`Poll::Ready`] when `await`'ed on.
///
/// # Examples
///
/// ```
/// use embassy_futures::block_on;
/// use embassy_mock::time::{MockTimer, Timer};
/// use embassy_time::Duration;
///
/// let timer = MockTimer::after(Duration::from_secs(1));
/// block_on(timer);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct MockTimer;

impl Future for MockTimer {
    type Output = ();

    /// Immediately return [`Poll::Ready`].
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

impl Timer for MockTimer {
    /// Create a [`MockTimer`] that can be used to unit test code.
    ///
    /// # Examples
    /// ```
    /// use embassy_mock::time::Timer;
    /// use embassy_time::Duration;
    ///
    /// async fn production_code<T: Timer>() {
    ///     // Do something...
    ///     T::after(Duration::from_millis(100)).await;
    ///     // Do something else...
    /// }
    ///
    /// # test_creating_timer();
    /// // The unit tests that use the `MockTimer`
    /// #[cfg(test)]
    /// mod tests {
    ///     use super::*;
    /// # }
    ///     use embassy_futures::block_on;
    ///     use embassy_mock::time::MockTimer;
    ///
    ///     #[test]
    ///     # fn hidden_fake_test(){}
    ///     fn test_creating_timer() {
    ///         block_on(production_code::<MockTimer>());
    ///     }
    /// # mod closing {
    /// }
    /// ```
    fn after(_duration: Duration) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embassy_futures::block_on;

    #[test]
    fn can_create_timer_with_after() {
        let timer = MockTimer::after(Duration::from_secs(1));

        assert_eq!(timer, MockTimer);
    }

    #[test]
    fn can_timer_impls_future() {
        let timer = MockTimer::after(Duration::from_secs(1));

        block_on(timer);
    }
}
