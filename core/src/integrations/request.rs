use crate::error::CoreResult;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};
use reqwest::{Response, Url};
use reqwest_middleware::ClientWithMiddleware;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

pub struct RequestBuilder<'a> {
    client: &'a ClientWithMiddleware,
    url: Url,
    headers: HeaderMap,
    rate_limiter: Option<Arc<leaky_bucket::RateLimiter>>,
    cost: usize,
}

impl<'a> RequestBuilder<'a> {
    pub fn new(client: &'a ClientWithMiddleware, url: impl AsRef<str>) -> CoreResult<Self> {
        Ok(Self {
            client,
            url: Url::parse(url.as_ref())?,
            headers: HeaderMap::new(),
            rate_limiter: None,
            cost: 1,
        })
    }

    pub fn rate_limiter(mut self, rate_limiter: &Arc<leaky_bucket::RateLimiter>) -> Self {
        self.rate_limiter = Some(rate_limiter.clone());
        self
    }

    pub fn header(mut self, key: impl IntoHeaderName, value: impl AsRef<str>) -> CoreResult<Self> {
        let header_value: HeaderValue = value.as_ref().parse()?;
        self.headers.insert(key, header_value);
        Ok(self)
    }

    pub fn query(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(key.as_ref(), value.as_ref());
        self
    }

    pub fn path(mut self, path: impl AsRef<str>) -> Self {
        self.url.set_path(path.as_ref());
        self
    }

    pub fn cost(mut self, cost: usize) -> Self {
        self.cost = cost;
        self
    }

    pub async fn get_json<T: DeserializeOwned>(self) -> CoreResult<T> {
        if let Some(rate_limiter) = self.rate_limiter {
            rate_limiter.acquire(self.cost).await
        }

        Ok(self
            .client
            .get(self.url)
            .headers(self.headers)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn post_form(self, form: impl Serialize) -> CoreResult<Response> {
        if let Some(rate_limiter) = self.rate_limiter {
            rate_limiter.acquire(self.cost).await
        };

        Ok(self
            .client
            .post(self.url)
            .headers(self.headers)
            .form(&form)
            .send()
            .await?
            .error_for_status()?)
    }
}
