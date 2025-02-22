pub use reqwest::Error;
use {
    core::fmt::Debug,
    http::{HeaderMap, Response},
    serde::{de::DeserializeOwned, Serialize},
    url::Url,
};

#[derive(Debug, Clone)]
pub struct Client(reqwest::Client);

impl Client {
    pub fn new(headers: HeaderMap) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self(client))
    }

    pub async fn post<T, R>(&self, url: Url, body: &T) -> Result<Response<R>, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let response = self.0.post(url).json(body).send().await?;
        let status = response.status();
        let body = response.json().await?;

        Ok(Response::builder().status(status).body(body).unwrap())
    }

    pub async fn get<R>(&self, _url: Url) -> Result<Response<R>, Error>
    where
        R: DeserializeOwned,
    {
        let response = self.0.get(_url).send().await?;
        let status = response.status();
        let body = response.json().await?;

        Ok(Response::builder().status(status).body(body).unwrap())
    }
}
