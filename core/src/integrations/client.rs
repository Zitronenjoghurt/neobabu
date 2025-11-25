use crate::error::CoreResult;
use crate::integrations::request::RequestBuilder;
use reqwest_leaky_bucket::leaky_bucket::RateLimiter;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use std::sync::Arc;
use std::time::Duration;

pub struct IntegrationClient {
    client: ClientWithMiddleware,
    rate_limiter: Arc<leaky_bucket::RateLimiter>,
}

impl IntegrationClient {
    pub fn new(max_tokens: usize, refill: usize, interval: Duration) -> Self {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        let limiter = RateLimiter::builder()
            .max(max_tokens)
            .initial(0)
            .refill(refill)
            .interval(interval)
            .build();

        Self {
            client,
            rate_limiter: Arc::new(limiter),
        }
    }

    pub fn request(&'_ self, base_url: impl AsRef<str>) -> CoreResult<RequestBuilder<'_>> {
        Ok(RequestBuilder::new(&self.client, base_url)?.rate_limiter(&self.rate_limiter))
    }
}
