use {
    core::fmt::{Debug, Display},
    http::{HeaderMap, Response},
    url::Url,
};

pub struct Error();

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "unimplemented reference http error")
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Error").finish()
    }
}
impl core::error::Error for Error {}

#[derive(Debug, Clone)]
pub struct Client {}

impl Client {
    pub fn new(_headers: HeaderMap) -> Result<Self, Error> {
        Err(Error())
    }

    pub async fn post<T, R>(&self, _url: Url, _body: &T) -> Result<Response<R>, Error> {
        Err(Error())
    }

    pub async fn get<R>(&self, _url: Url) -> Result<Response<R>, Error> {
        Err(Error())
    }
}
