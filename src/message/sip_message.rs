pub use tokenizer::Tokenizer;

use super::{request, response};
use crate::{common::Version, Error, Headers, Request, Response};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SipMessage {
    Request(Request),
    Response(Response),
}

impl SipMessage {
    pub fn is_request(&self) -> bool {
        matches!(self, Self::Request(_))
    }

    pub fn is_response(&self) -> bool {
        matches!(self, Self::Response(_))
    }

    pub fn version(&self) -> &Version {
        match self {
            Self::Request(request) => request.version(),
            Self::Response(response) => response.version(),
        }
    }

    pub fn headers(&self) -> &Headers {
        match self {
            Self::Request(request) => request.headers(),
            Self::Response(response) => response.headers(),
        }
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        match self {
            Self::Request(request) => request.headers_mut(),
            Self::Response(response) => response.headers_mut(),
        }
    }

    pub fn body(&self) -> &Vec<u8> {
        match self {
            Self::Request(request) => request.body(),
            Self::Response(response) => response.body(),
        }
    }

    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        match self {
            Self::Request(request) => request.body_mut(),
            Self::Response(response) => response.body_mut(),
        }
    }
}

impl TryFrom<&[u8]> for SipMessage {
    type Error = Error;

    fn try_from(from: &[u8]) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(from)?.1.try_into()
    }
}

impl TryFrom<Vec<u8>> for SipMessage {
    type Error = Error;

    fn try_from(from: Vec<u8>) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(&from)?.1.try_into()
    }
}

impl TryFrom<&str> for SipMessage {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(from.as_bytes())?.1.try_into()
    }
}

impl TryFrom<String> for SipMessage {
    type Error = Error;

    fn try_from(from: String) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(&from.as_bytes())?.1.try_into()
    }
}

impl TryFrom<bytes::Bytes> for SipMessage {
    type Error = Error;

    fn try_from(from: bytes::Bytes) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(&from)?.1.try_into()
    }
}

pub mod tokenizer {
    use super::{request, response, SipMessage};
    use crate::{Error, NomError};
    use std::convert::TryInto;

    impl<'a> TryInto<SipMessage> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<SipMessage, Error> {
            match self {
                Tokenizer::Request(tokenizer) => Ok(SipMessage::Request(tokenizer.try_into()?)),
                Tokenizer::Response(tokenizer) => Ok(SipMessage::Response(tokenizer.try_into()?)),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Tokenizer<'a> {
        Request(request::Tokenizer<'a>),
        Response(response::Tokenizer<'a>),
    }

    impl<'a> From<request::Tokenizer<'a>> for Tokenizer<'a> {
        fn from(tokenizer: request::Tokenizer<'a>) -> Self {
            Self::Request(tokenizer)
        }
    }

    impl<'a> From<response::Tokenizer<'a>> for Tokenizer<'a> {
        fn from(tokenizer: response::Tokenizer<'a>) -> Self {
            Self::Response(tokenizer)
        }
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use nom::{branch::alt, combinator::map};

            let (_, message) = alt((
                map(response::Tokenizer::tokenize, |r| r.into()),
                map(request::Tokenizer::tokenize, |r| r.into()),
            ))(part)?;

            Ok((&[], message))
        }
    }
}
