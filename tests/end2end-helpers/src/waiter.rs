use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use thirtyfour::{
    error::{WebDriverError, WebDriverResult},
    extensions::query::{ElementPollerWithTimeout, IntoElementPoller},
};

use async_trait::async_trait;

#[async_trait]
pub trait Predicate: Send + Sync {
    async fn call(&self) -> WebDriverResult<bool>;
}

#[async_trait]
impl<F, Fut> Predicate for F
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = WebDriverResult<bool>> + Send,
{
    async fn call(&self) -> WebDriverResult<bool> {
        (self)().await
    }
}

#[derive(Debug)]
pub struct Waiter {
    poller: Arc<dyn IntoElementPoller + Send + Sync>,
    message: String,
}

impl Waiter {
    /// Create a new `Waiter`.
    pub fn new(timeout: Duration, interval: Duration, message: String) -> Self {
        let poller = Arc::new(ElementPollerWithTimeout::new(timeout, interval));
        Self { poller, message }
    }

    async fn run_poller<'a, F, I, P>(
        &self,
        conditions: F,
    ) -> WebDriverResult<bool>
    where
        F: Fn() -> I,
        I: IntoIterator<Item = &'a P>,
        P: Predicate + ?Sized + 'a,
    {
        let mut poller = self.poller.start();
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

    fn timeout(self) -> WebDriverResult<()> {
        Err(WebDriverError::Timeout(format!(
            "condition timed out: {}",
            self.message
        )))
    }

    pub async fn until<'a, F, I, P>(self, conditions: F) -> WebDriverResult<()>
    where
        F: Fn() -> I,
        I: IntoIterator<Item = &'a P>,
        P: Predicate + ?Sized + 'a,
    {
        let success = self.run_poller(conditions).await?;
        if success { Ok(()) } else { self.timeout() }
    }
}
