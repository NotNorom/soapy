use std::{error::Error as StdError, fmt::Debug};

use thiserror::Error;

use crate::SoapResponseBody;

pub trait DebugSoapResponseBody: SoapResponseBody + Debug {}
impl<T> DebugSoapResponseBody for T where T: SoapResponseBody + Debug {}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error sending request to target={target}, error={error}, body={body}")]
    SendRequest {
        target: String,
        body: String,
        #[source]
        error: reqwest::Error,
    },
    #[error("Error decoding resonse from target={target}, error={error}, body={body}")]
    Decode {
        target: String,
        body: String,
        #[source]
        error: xml_serde::Error,
    },
    #[error("Unexpected reponse variant")]
    UnexpectedResponseVariant {
        variant: Box<dyn DebugSoapResponseBody>,
    },
    #[error("{0:?}")]
    Fault(Box<dyn DebugSoapResponseBody>),
}

impl Error {
    #[inline]
    pub fn new_fault<T: DebugSoapResponseBody + 'static>(fault: T) -> Self {
        Self::Fault(Box::new(fault))
    }

    #[inline]
    pub fn new_unexpected_variant<T: DebugSoapResponseBody + 'static>(variant: T) -> Self {
        Self::UnexpectedResponseVariant {
            variant: Box::new(variant),
        }
    }

    #[inline]
    pub fn root_cause(&self) -> &dyn StdError {
        let mut error: &dyn StdError = self;

        while let Some(err) = error.source() {
            error = err;
        }

        error
    }
}
