use core::sync::atomic::{AtomicUsize, Ordering};
use embassy_executor::{SpawnError, SpawnToken, Spawner as EmbassySpawner};

pub trait Spawner {
    fn spawn<S>(&self, token: SpawnToken<S>) -> Result<(), SpawnError>;
}

impl Spawner for EmbassySpawner {
    fn spawn<S>(&self, token: SpawnToken<S>) -> Result<(), SpawnError> {
        self.spawn(token)
    }
}

#[derive(Debug)]
pub struct MockSpawner<const N: usize> {
    times_called: AtomicUsize,
}

impl<const N: usize> MockSpawner<N> {
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
