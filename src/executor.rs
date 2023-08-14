//! A mocked version of the `embassy-executor` crate.
//!
//! # Examples
//! ```
//! # #![feature(type_alias_impl_trait)]
//! #
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
//!         let spawner = MockSpawner::<1>::new();
//!         spawn_tasks(&spawner);
//!     }
//! # mod closing {
//! }
//! ```

use core::sync::atomic::{AtomicUsize, Ordering};
use embassy_executor::{SpawnError, SpawnToken, Spawner as EmbassySpawner};

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

/// A mocked version of [`embassy_executor::Spawner`] that can be used in its place for unit tests.
///
/// This mocked version counts how many times [`Self::spawn()`] is called and asserts that it was
/// called the correct number of times when dropped.
///
/// The const generic argument `N` is used to declare how many times [`Self::spawn()`] should be
/// called.
///
/// # Examples
///
/// ```
/// # #![feature(type_alias_impl_trait)]
/// #
/// use embassy_mock::executor::{MockSpawner, Spawner};
///
/// #[embassy_executor::task]
/// async fn example_task() {}
///
/// let spawner = MockSpawner::<1>::new();  // Expects `spawn()` to be called once.
/// spawner.spawn(example_task()).unwrap(); // `spawn()` is called once.
/// ```
///
/// ```should_panic
/// # #![feature(type_alias_impl_trait)]
/// #
/// use embassy_mock::executor::{MockSpawner, Spawner};
///
/// #[embassy_executor::task]
/// async fn example_task() {}
///
/// let spawner = MockSpawner::<2>::new();  // Expects `spawn()` to be called twice.
/// spawner.spawn(example_task()).unwrap(); // `spawn()` is called only once.
/// ```
#[derive(Debug)]
pub struct MockSpawner<const N: usize> {
    times_called: AtomicUsize,
}

impl<const N: usize> MockSpawner<N> {
    /// Create a new [`MockSpawner`].
    ///
    /// # Examples
    ///
    /// ```
    /// use embassy_mock::executor::MockSpawner;
    ///
    /// # const X: usize = 0;
    /// let spawner = MockSpawner::<X>::new(); // Where `X` is the number of times `spawn()` should be called
    /// ```
    pub const fn new() -> Self {
        Self {
            times_called: AtomicUsize::new(0),
        }
    }
}

impl<const N: usize> Drop for MockSpawner<N> {
    fn drop(&mut self) {
        let times_called = self.times_called.load(Ordering::Relaxed);
        assert_eq!(
            N, times_called,
            "expected to spawn {} task(s), actually spawned {}",
            N, times_called
        );
    }
}

impl<const N: usize> Spawner for MockSpawner<N> {
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
    fn can_spawn_single_task() {
        let spawner = MockSpawner::<1>::new();
        spawner.spawn(example_task()).unwrap();
    }

    #[test]
    fn can_spawn_multiple_tasks() {
        let spawner = MockSpawner::<3>::new();
        spawner.spawn(example_task()).unwrap();
        spawner.spawn(example_task()).unwrap();
        spawner.spawn(example_task()).unwrap();
    }

    #[test]
    #[should_panic(expected = "expected to spawn 1 task(s), actually spawned 3")]
    fn spawn_too_many_tasks() {
        let spawner = MockSpawner::<1>::new();
        spawner.spawn(example_task()).unwrap();
        spawner.spawn(example_task()).unwrap();
        spawner.spawn(example_task()).unwrap();
    }

    #[test]
    #[should_panic(expected = "expected to spawn 3 task(s), actually spawned 1")]
    fn spawn_too_few_tasks() {
        let spawner = MockSpawner::<3>::new();
        spawner.spawn(example_task()).unwrap();
    }
}
