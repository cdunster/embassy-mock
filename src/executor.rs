//! A mocked version of the `embassy-executor` crate.
//!
//! # Examples
//! ```
//! use embassy_mock::executor::Spawner;
//!
//! #[embassy_executor::task]
//! async fn example_task() {}
//!
//! // Generic over the `Spawner` trait
//! pub fn spawn_tasks<S: Spawner>(spawner: &S) {
//!     spawner.spawn(example_task()).unwrap();
//! }
//!
//! // The real main that runs on the Embassy executor.
//! // #[embassy_executor::main]
//! // async fn main(spawner: embassy_executor::Spawner) {
//! //     spawn_tasks(&spawner);
//! // }
//!
//! # test_spawning_of_tasks();
//! // The unit tests that use the `MockSpawner`.
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//! # }
//!     use embassy_mock::executor::MockSpawner;
//!
//!     #[test]
//!     # fn hidden_fake_test(){}
//!     fn test_spawning_of_tasks() {
//!         let spawner = MockSpawner::expect(1);
//!         spawn_tasks(&spawner);
//!
//!         assert_eq!(spawner.done(), Ok(()));
//!     }
//! # mod closing {
//! }
//! ```

use core::sync::atomic::{AtomicUsize, Ordering};
use embassy_executor::{SpawnError, SpawnToken, Spawner as EmbassySpawner};
use snafu::prelude::*;

/// The trait to replace the [`embassy_executor::Spawner`] in code to allow the [`MockSpawner`] to
/// be used in its place for tests.
pub trait Spawner {
    /// Wrapper for [`embassy_executor::Spawner::spawn()`].
    fn spawn<S>(&self, token: SpawnToken<S>) -> Result<(), SpawnError>;
}

impl Spawner for EmbassySpawner {
    /// Spawn a task into an executor.
    ///
    /// You obtain the `token` by calling a task function (i.e. one marked with `#[embassy_executor::task]`).
    fn spawn<S>(&self, token: SpawnToken<S>) -> Result<(), SpawnError> {
        self.spawn(token)
    }
}

/// The errors that are reported by [`MockSpawner`].
#[derive(Debug, Snafu, PartialEq)]
pub enum MockSpawnerError {
    /// The [`MockSpawner::spawn()`] method was called the wrong number of times.
    #[snafu(display("expected to spawn {expected} task(s), actually spawned {actual}"))]
    WrongNumberOfTasks {
        /// The expected number of calls to [`MockSpawner::spawn()`].
        expected: usize,

        /// The actual number of times [`MockSpawner::spawn()`] was called.
        actual: usize,
    },
}

/// A mocked version of [`embassy_executor::Spawner`] that can be used in its place for unit tests.
///
/// This mocked version counts how many times [`Self::spawn()`] is called and can be checked that
/// [`Self::spawn()`] was called the correct number of times using [`Self::done()`]. If
/// [`Self::done()`] is not called then it asserts that [`Self::spawn()`] was called the correct
/// number of times when dropped which causes a panic if incorrect.
///
/// # Panics
///
/// Panics if [`Self::spawn()`] called the wrong number of times and [`Self`] is dropped before
/// calling [`Self::done()`].
///
/// # Examples
///
/// ```
/// use embassy_mock::executor::{MockSpawner, MockSpawnerError, Spawner};
///
/// #[embassy_executor::task]
/// async fn example_task() {}
///
/// let spawner = MockSpawner::expect(4);
/// spawner.spawn(example_task()).unwrap();
///
/// let res = spawner.done();
///
/// let expected = Err(MockSpawnerError::WrongNumberOfTasks {
///     expected: 4,
///     actual: 1,
/// });
/// assert_eq!(res, expected);
/// ```
///
/// ```
/// use embassy_mock::executor::{MockSpawner, Spawner};
///
/// #[embassy_executor::task]
/// async fn example_task() {}
///
/// let spawner = MockSpawner::expect(1); // Expects `spawn()` to be called once.
/// spawner.spawn(example_task()).unwrap(); // `spawn()` is called once.
///
/// // `spawner` is dropped but doesn't panic.
/// ```
///
/// ```should_panic
/// use embassy_mock::executor::{MockSpawner, Spawner};
///
/// #[embassy_executor::task]
/// async fn example_task() {}
///
/// let spawner = MockSpawner::expect(2); // Expects `spawn()` to be called twice.
/// spawner.spawn(example_task()).unwrap(); // `spawn()` is called only once.
///
/// // `spawner` is dropped and will panic.
/// ```
#[derive(Debug)]
pub struct MockSpawner {
    /// The number of expected calls to [`Self::spawn()`].
    expected: usize,

    /// The number of times [`Self::spawn()`] has been called.
    times_called: AtomicUsize,

    /// Has this mock been checked with a call to [`Self::done()`].
    /// If true it is not checked when dropped.
    is_done: bool,
}

impl MockSpawner {
    /// Create a [`MockSpawner`], providing the expected number of calls to [`Self::spawn()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use embassy_mock::executor::MockSpawner;
    ///
    /// # const X: usize = 0;
    /// let spawner = MockSpawner::expect(X); // Where `X` is the number of times `spawn()` should be called
    /// ```
    pub const fn expect(expected: usize) -> Self {
        Self {
            expected,
            times_called: AtomicUsize::new(0),
            is_done: false,
        }
    }

    /// Mark the [`MockSpawner`] as done and check if [`Self::spawn()`] was called the correct
    /// number of times.
    ///
    /// This is a cleaner way of testing that [`Self::spawn()`] is called the correct number of
    /// times as [`MockSpawner`] doesn't cause a panic when dropped if this method is called,
    /// it also returns a [`Result<(), MockSpawnerError>`] which allows checking the outcome of the
    /// mock.
    ///
    /// # Examples
    ///
    /// ```
    /// use embassy_mock::executor::{MockSpawner, Spawner};
    ///
    /// #[embassy_executor::task]
    /// async fn example_task() {}
    ///
    /// let spawner = MockSpawner::expect(1);
    /// spawner.spawn(example_task()).unwrap();
    ///
    /// let res = spawner.done();
    ///
    /// assert_eq!(res, Ok(()));
    /// ```
    ///
    /// ```
    /// use embassy_mock::executor::{MockSpawner, MockSpawnerError, Spawner};
    ///
    /// #[embassy_executor::task]
    /// async fn example_task() {}
    ///
    /// let spawner = MockSpawner::expect(4);
    /// spawner.spawn(example_task()).unwrap();
    ///
    /// let res = spawner.done();
    ///
    /// let expected = Err(MockSpawnerError::WrongNumberOfTasks {
    ///     expected: 4,
    ///     actual: 1,
    /// });
    /// assert_eq!(res, expected);
    ///
    /// // This doesn't panic when `spawner` is dropped as `spawner.done()` was called.
    /// ```
    pub fn done(mut self) -> Result<(), MockSpawnerError> {
        let times_called = self.times_called.load(Ordering::Relaxed);
        let res = if times_called != self.expected {
            Err(MockSpawnerError::WrongNumberOfTasks {
                expected: self.expected,
                actual: times_called,
            })
        } else {
            Ok(())
        };

        self.is_done = true;
        res
    }
}

impl Drop for MockSpawner {
    /// If [`Self::done()`] has not been called before being dropped then check that the number of
    /// times [`Self::spawn()`] was called is as expected.
    fn drop(&mut self) {
        if !self.is_done {
            let times_called = self.times_called.load(Ordering::Relaxed);
            assert_eq!(
                self.expected, times_called,
                "expected to spawn {} task(s), actually spawned {}",
                self.expected, times_called
            );
        }
    }
}

impl Spawner for MockSpawner {
    /// Increment an internal counter of how many times this method is called.
    fn spawn<S>(&self, token: SpawnToken<S>) -> Result<(), SpawnError> {
        // Need to forget the token so that it is not dropped which causes a panic
        core::mem::forget(token);
        let times_called = self
            .times_called
            .load(Ordering::Relaxed)
            .checked_add(1)
            .unwrap();
        self.times_called.store(times_called, Ordering::Relaxed);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[embassy_executor::task]
    async fn example_task() {}

    #[test]
    fn can_spawn_single_task_just_drop() {
        let spawner = MockSpawner::expect(1);
        spawner.spawn(example_task()).unwrap();
    }

    #[test]
    fn can_spawn_multiple_tasks_just_drop() {
        let spawner = MockSpawner::expect(3);
        spawner.spawn(example_task()).unwrap();
        spawner.spawn(example_task()).unwrap();
        spawner.spawn(example_task()).unwrap();
    }

    #[test]
    #[should_panic(expected = "expected to spawn 1 task(s), actually spawned 3")]
    fn spawn_too_many_tasks_just_drop() {
        let spawner = MockSpawner::expect(1);
        spawner.spawn(example_task()).unwrap();
        spawner.spawn(example_task()).unwrap();
        spawner.spawn(example_task()).unwrap();
    }

    #[test]
    #[should_panic(expected = "expected to spawn 3 task(s), actually spawned 1")]
    fn spawn_too_few_tasks_just_drop() {
        let spawner = MockSpawner::expect(3);
        spawner.spawn(example_task()).unwrap();
    }

    #[test]
    fn done_returns_ok() {
        let spawner = MockSpawner::expect(1);
        spawner.spawn(example_task()).unwrap();

        let res = spawner.done();

        assert_eq!(res, Ok(()));
    }

    #[test]
    fn done_returns_err_does_not_panic_on_drop() {
        let spawner = MockSpawner::expect(3);
        spawner.spawn(example_task()).unwrap();

        let res = spawner.done();

        let expected = Err(MockSpawnerError::WrongNumberOfTasks {
            expected: 3,
            actual: 1,
        });
        assert_eq!(res, expected);
    }
}
