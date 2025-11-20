use crate::error::CoreResult;
use crate::integrations::request::RequestBuilder;
use reqwest_leaky_bucket::leaky_bucket::RateLimiter;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use std::time::Duration;

pub struct IntegrationClient {
    client: ClientWithMiddleware,
}

impl IntegrationClient {
    pub fn new(max_tokens: usize, refill: usize, interval: Duration) -> Self {
        let limiter = RateLimiter::builder()
            .max(max_tokens)
            .initial(0)
            .refill(refill)
            .interval(interval)
            .build();
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);

        let client = ClientBuilder::new(reqwest::Client::new())
            .with(reqwest_leaky_bucket::rate_limit_all(limiter))
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Self { client }
    }

    pub fn request(&'_ self, base_url: impl AsRef<str>) -> CoreResult<RequestBuilder<'_>> {
        RequestBuilder::new(&self.client, base_url)
    }
}
