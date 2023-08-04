pub mod error;
pub mod generator;

pub use error::Error;

use serde::{Deserialize, Serialize};
use tracing::debug;
use xml_serde::from_str;

pub trait SoapHeader {}
impl<'a, T> SoapHeader for T where T: Serialize + Deserialize<'a> {}

pub mod request {
    use serde::{Deserialize, Serialize};

    use crate::SoapHeader;

    pub trait SoapRequestBody {}
    impl<T> SoapRequestBody for T where T: Serialize {}

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Request<T: SoapRequestBody, U: SoapHeader = ()> {
        #[serde(rename = "{http://schemas.xmlsoap.org/soap/envelope/}S:Envelope")]
        pub envelope: RequestEnvelope<T, U>,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct RequestEnvelope<T: SoapRequestBody, U: SoapHeader> {
        #[serde(
            rename = "{http://schemas.xmlsoap.org/soap/envelope/}S:Header",
            skip_serializing_if = "Option::is_none"
        )]
        pub header: Option<U>,
        #[serde(rename = "{http://schemas.xmlsoap.org/soap/envelope/}S:Body")]
        pub body: T,
    }
}

pub mod response {
    use serde::{Deserialize, Serialize};

    use crate::SoapHeader;

    pub trait SoapResponseBody {
        #[inline]
        fn fault(&self) -> Option<Fault> {
            None
        }

        #[inline]
        fn body(&self) -> Option<Self>
        where
            Self: Sized,
        {
            None
        }
    }

    impl<'a, T> SoapResponseBody for T where T: Deserialize<'a> {}

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Response<T: SoapResponseBody, U: SoapHeader = ()> {
        #[serde(rename = "{http://schemas.xmlsoap.org/soap/envelope/}S:Envelope")]
        pub envelope: ResponseEnvelope<T, U>,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct ResponseEnvelope<T: SoapResponseBody, U: SoapHeader> {
        #[serde(
            rename = "{http://schemas.xmlsoap.org/soap/envelope/}S:Header",
            skip_serializing_if = "Option::is_none"
        )]
        pub header: Option<U>,
        #[serde(rename = "{http://schemas.xmlsoap.org/soap/envelope/}S:Body")]
        pub body: T,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Fault {
        faultcode: FaultCode,
        faultstring: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        faultactor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        detail: Option<FaultDetail>,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    #[non_exhaustive]
    pub enum FaultCode {
        #[serde(rename = "S:VersionMismatch")]
        VersionMismatch,
        #[serde(rename = "S:MustUnderstand")]
        MustUnderstand,
        #[serde(rename = "S:Client")]
        Client,
        #[serde(rename = "S:Server")]
        Server,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    #[non_exhaustive]
    pub enum FaultDetail {
        #[serde(rename = "$value")]
        String(String),
        #[serde(rename = "{http://jax-ws.dev.java.net/}ns2:exception")]
        Exception(Exception),
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Exception {
        #[serde(rename = "$attr:class")]
        class: String,
        #[serde(rename = "$attr:note")]
        note: String,
        #[serde(rename = "{http://jax-ws.dev.java.net/}ns2:stackTrace")]
        stacktrace: Stacktrace,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Stacktrace {
        #[serde(rename = "{http://jax-ws.dev.java.net/}ns2:frame")]
        frame: Vec<Frame>,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Frame {
        #[serde(rename = "$attr:class")]
        class: String,
        #[serde(rename = "$attr:file")]
        file: Option<String>,
        #[serde(rename = "$attr:line")]
        line: String,
        #[serde(rename = "$attr:method")]
        method: String,
    }
}

#[derive(Debug, Default, Clone)]
pub struct Client {
    http: reqwest::Client,
}

impl Client {
    #[inline]
    #[must_use]
    pub const fn new(client: reqwest::Client) -> Self {
        Self { http: client }
    }

    #[inline]
    pub async fn send<T>(&self, body: &str, target: &str) -> Result<T, Error>
    where
        T: for<'a> Deserialize<'a>,
    {
        let raw_response = self
            .post_request(body, target)
            .await
            .map_err(|err| Error::SendRequest {
                target: target.to_owned(),
                body: body.to_owned(),
                error: err,
            })?;
        let soap_resp: Response<T> = from_str(&raw_response).map_err(|err| Error::Decode {
            target: target.to_owned(),
            body: raw_response,
            error: err,
        })?;
        Ok(soap_resp.envelope.body)
    }

    async fn post_request(&self, body: &str, target: impl reqwest::IntoUrl) -> Result<String, reqwest::Error> {
        self.http
            .post(target.into_url()?)
            .body(body.to_owned())
            .send()
            .await?
            .text()
            .await
            .map(|raw_response| {
                debug!("{raw_response}");
                raw_response
            })
    }
}

pub use crate::{request::*, response::*};
