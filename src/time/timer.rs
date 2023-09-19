//! Traits and mocked types to allow unit testing functions that require an
//! [`embassy_time::Timer`].
//!
//! # Examples
//! ```
//! # #![feature(type_alias_impl_trait)]
//! #
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
use embassy_sync::channel::Receiver;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{Duration, Timer as EmbassyTimer};
use snafu::Snafu;

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
    /// # Examples:
    /// ``` no_run
    /// # #![feature(type_alias_impl_trait)]
    /// #
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

/// The errors that are reported by [`MockTimer`].
#[derive(Debug, Snafu, PartialEq)]
pub enum MockTimerError {
    /// The [`MockTimer::after()`] associated function was last called with the wrong duration.
    #[snafu(display("expected to call with {expected} duration, actual {actual}"))]
    WrongDuration {
        /// The expected duration passed to [`MockTimer::after()`].
        expected: Duration,

        /// The actual duration [`MockTimer::after()`] was called with.
        actual: Duration,
    },
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
pub struct MockTimer {
    duration: Duration,
}

/// A [`Channel`] to send the [`Duration`]'s that are passed into calls to [`MockTimer::after`].
///
/// # Note
/// Shared for all [`MockTimer`]'s.
static DURATION_CHANNEL: Channel<CriticalSectionRawMutex, Duration, 5> = Channel::new();

impl MockTimer {
    /// Get a [`Receiver`] for receiving durations passed into calls to [`Self::after`] this
    /// [`Receiver`] is for a [`Channel`] that is shared for all [`MockTimer`]'s so running tests
    /// in parallel may cause unexpected results.
    ///
    /// # Examples
    /// ```
    /// use embassy_futures::block_on;
    /// use embassy_mock::time::{MockTimer, Timer};
    /// use embassy_time::Duration;
    ///
    /// let timer1 = MockTimer::after(Duration::from_millis(500));
    /// let timer2 = MockTimer::after(Duration::from_secs(1));
    ///
    /// // Even though timer1 is created first, timer2 is blocked on first so will be the first
    /// // Duration in the channel.
    /// block_on(timer2);
    /// block_on(timer1);
    ///
    /// let rx = MockTimer::get_receiver();
    /// assert_eq!(rx.try_recv(), Ok(Duration::from_secs(1)));
    /// assert_eq!(rx.try_recv(), Ok(Duration::from_millis(500)));
    /// ```
    pub fn get_receiver() -> Receiver<'static, CriticalSectionRawMutex, Duration, 5> {
        DURATION_CHANNEL.receiver()
    }

    /// Clear the [`Channel`] of all messages.
    ///
    /// # Examples
    /// ```
    /// use embassy_futures::block_on;
    /// use embassy_mock::time::{MockTimer, Timer};
    /// use embassy_sync::channel::TryRecvError;
    /// use embassy_time::Duration;
    ///
    /// // Awaited on timer so message gets sent on the channel
    /// block_on(MockTimer::after(Duration::from_millis(500)));
    ///
    /// // We don't care about previous values so just clear the channel
    /// MockTimer::clear_channel();
    ///
    /// block_on(MockTimer::after(Duration::from_secs(1)));
    ///
    /// let rx = MockTimer::get_receiver();
    /// assert_eq!(rx.try_recv(), Ok(Duration::from_secs(1)));
    /// assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
    /// ```
    pub fn clear_channel() {
        while DURATION_CHANNEL.try_recv().is_ok() {}
    }
}

impl Future for MockTimer {
    type Output = ();

    /// Send the value of `self.duration` via the channel and return [`Poll::Ready`].
    ///
    /// # Note
    /// All send errors are ignored as using the channel is not a requirement.
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let _ = DURATION_CHANNEL.try_send(self.duration);
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
    fn after(duration: Duration) -> Self {
        Self { duration }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embassy_futures::block_on;
    use embassy_sync::channel::TryRecvError;
    use serial_test::serial;

    #[test]
    fn can_create_timer_with_after() {
        let timer = MockTimer::after(Duration::from_secs(1));

        assert_eq!(
            timer,
            MockTimer {
                duration: Duration::from_secs(1)
            }
        );
    }

    #[test]
    #[serial]
    fn can_get_duration_via_channel_when_awaited() {
        MockTimer::clear_channel();

        block_on(MockTimer::after(Duration::from_secs(5))); // Same as calling `.await`

        let rx = MockTimer::get_receiver();
        assert_eq!(rx.try_recv(), Ok(Duration::from_secs(5)));
    }

    #[test]
    #[serial]
    fn duration_not_sent_on_channel_when_not_awaited() {
        MockTimer::clear_channel();

        let _timer = MockTimer::after(Duration::from_secs(2)); // Timer created but not awaited

        let rx = MockTimer::get_receiver();
        assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
    }

    #[test]
    #[serial]
    fn can_get_multiple_durations_via_channel() {
        MockTimer::clear_channel();

        let timer1 = MockTimer::after(Duration::from_millis(500));
        let timer2 = MockTimer::after(Duration::from_secs(1));

        // Even though timer1 is created first, timer2 is blocked on first so will be the first
        // Duration in the channel.
        block_on(timer2);
        block_on(timer1);

        let rx = MockTimer::get_receiver();
        assert_eq!(rx.try_recv(), Ok(Duration::from_secs(1)));
        assert_eq!(rx.try_recv(), Ok(Duration::from_millis(500)));
    }

    #[test]
    #[serial]
    fn channel_send_errors_are_ignored() {
        MockTimer::clear_channel();

        // Call the timer 6 times as the channel can only store 5 messages.
        for _ in 0..6 {
            block_on(MockTimer::after(Duration::from_millis(10)));
        }
    }
}
