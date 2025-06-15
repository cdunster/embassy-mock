#![no_std]

use embassy_mock::executor::Spawner;
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
async fn task_a() -> ! {
    let mut val = 0_u64;
    loop {
        val = val.wrapping_add(1);
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn task_b(wait_for: Duration) {
    Timer::after(wait_for).await;
}

pub fn spawn_tasks<S: Spawner>(spawner: &S) {
    spawner.spawn(task_a()).unwrap();
    spawner.spawn(task_b(Duration::from_secs(5))).unwrap();
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    spawn_tasks(&spawner);
}

#[cfg(test)]
mod tests {
    use super::*;
    use embassy_mock::executor::MockSpawner;

    #[test]
    fn spawn_tasks_spawns_all_tasks() {
        let spawner = MockSpawner::expect(2);
        spawn_tasks(&spawner);

        spawner.done().unwrap();
    }
}
