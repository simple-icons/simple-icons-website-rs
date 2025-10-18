use anyhow::Result;
use async_trait::async_trait;
use std::future::Future;
use std::time::{Duration, Instant};

#[async_trait]
pub trait Predicate: Send + Sync {
    async fn call(&self) -> Result<bool>;
}

#[async_trait]
impl<F, Fut> Predicate for F
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = Result<bool>> + Send,
{
    async fn call(&self) -> Result<bool> {
        (self)().await
    }
}

pub struct PollerWithTimeout {
    timeout: Duration,
    interval: Duration,
    start: Instant,
    cur_tries: u32,
}

impl PollerWithTimeout {
    /// Create a new `PollerWithTimeout`.
    pub fn new(timeout: Duration, interval: Duration) -> Self {
        Self {
            timeout,
            interval,
            start: Instant::now(),
            cur_tries: 0,
        }
    }
}

#[async_trait::async_trait]
pub trait Poller {
    /// Process the poller forward by one tick.
    async fn tick(&mut self) -> bool;
}

#[async_trait::async_trait]
impl Poller for PollerWithTimeout {
    async fn tick(&mut self) -> bool {
        self.cur_tries += 1;

        if self.start.elapsed() >= self.timeout {
            return false;
        }

        // The Next poll is due no earlier than this long after the first poll started.
        let minimum_elapsed = self.interval.saturating_mul(self.cur_tries);

        // But this much time has elapsed since the first poll started.
        let actual_elapsed = self.start.elapsed();

        if actual_elapsed < minimum_elapsed {
            // So we need to wait this much longer.
            tokio::time::sleep(minimum_elapsed - actual_elapsed).await;
        }

        true
    }
}

pub struct Waiter {
    timeout: Duration,
    interval: Duration,
    message: String,
}

impl Waiter {
    /// Create a new `Waiter`.
    pub fn new(timeout: Duration, interval: Duration, message: String) -> Self {
        Self {
            timeout,
            interval,
            message,
        }
    }

    async fn run_poller<'a, F, I, P>(&self, conditions: F) -> Result<bool>
    where
        F: Fn() -> I,
        I: IntoIterator<Item = &'a P>,
        P: Predicate + ?Sized + 'a,
    {
        let mut poller = PollerWithTimeout::new(self.timeout, self.interval);
        loop {
            let mut conditions_met = true;
            for f in conditions() {
                if !f.call().await? {
                    conditions_met = false;
                    break;
                }
            }

            if conditions_met {
                return Ok(true);
            }

            if !poller.tick().await {
                return Ok(false);
            }
        }
    }

    /// Wait until all the provided conditions are met or timeout occurs.
    pub async fn until<'a, F, I, P>(self, conditions: F) -> Result<()>
    where
        F: Fn() -> I,
        I: IntoIterator<Item = &'a P>,
        P: Predicate + ?Sized + 'a,
    {
        let success = self.run_poller(conditions).await?;
        if success {
            Ok(())
        } else {
            Err(anyhow::anyhow!(self.message))
        }
    }
}
