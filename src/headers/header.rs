use crate::headers::untyped::*;

#[doc(hidden)]
pub use tokenizer::Tokenizer;

/// Big enum holding in each variant every possible header defined in SIP. All variants hold an
/// [untyped](super::untyped) header which is basically a NewType around String.
///
/// In case a header is not defined in Rsip, parsing will store it in the `Other`
/// variant, which is a tuple of Strings.
/// For instance, constructing the `X-Fs-Sending-Message` header (related to SMS in SIP),
/// you can do:
/// ```
/// let x_fs_sending_message = rsip::Header::Other("X-FS-Sending-Message".into(), "f9c4adc8-9c2a-47d5-a7f1-63d20784685e".into());
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Header {
    Accept(Accept),
    AcceptEncoding(AcceptEncoding),
    AcceptLanguage(AcceptLanguage),
    AlertInfo(AlertInfo),
    Allow(Allow),
    AuthenticationInfo(AuthenticationInfo),
    Authorization(Authorization),
    CSeq(CSeq),
    CallId(CallId),
    CallInfo(CallInfo),
    Contact(Contact),
    ContentDisposition(ContentDisposition),
    ContentEncoding(ContentEncoding),
    ContentLanguage(ContentLanguage),
    ContentLength(ContentLength),
    ContentType(ContentType),
    Date(Date),
    ErrorInfo(ErrorInfo),
    Event(Event),
    Expires(Expires),
    From(From),
    InReplyTo(InReplyTo),
    MaxForwards(MaxForwards),
    MimeVersion(MimeVersion),
    MinExpires(MinExpires),
    Organization(Organization),
    Other(String, String),
    Priority(Priority),
    ProxyAuthenticate(ProxyAuthenticate),
    ProxyAuthorization(ProxyAuthorization),
    ProxyRequire(ProxyRequire),
    RecordRoute(RecordRoute),
    ReplyTo(ReplyTo),
    Require(Require),
    RetryAfter(RetryAfter),
    Route(Route),
    Server(Server),
    Subject(Subject),
    SubscriptionState(SubscriptionState),
    Supported(Supported),
    Timestamp(Timestamp),
    To(To),
    Unsupported(Unsupported),
    UserAgent(UserAgent),
    Via(Via),
    Warning(Warning),
    WwwAuthenticate(WwwAuthenticate),
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Accept(inner) => write!(f, "{}", inner),
            Self::AcceptEncoding(inner) => write!(f, "{}", inner),
            Self::AcceptLanguage(inner) => write!(f, "{}", inner),
            Self::AlertInfo(inner) => write!(f, "{}", inner),
            Self::Allow(inner) => write!(f, "{}", inner),
            Self::AuthenticationInfo(inner) => write!(f, "{}", inner),
            Self::Authorization(inner) => write!(f, "{}", inner),
            Self::CSeq(inner) => write!(f, "{}", inner),
            Self::CallId(inner) => write!(f, "{}", inner),
            Self::CallInfo(inner) => write!(f, "{}", inner),
            Self::Contact(inner) => write!(f, "{}", inner),
            Self::ContentDisposition(inner) => write!(f, "{}", inner),
            Self::ContentEncoding(inner) => write!(f, "{}", inner),
            Self::ContentLanguage(inner) => write!(f, "{}", inner),
            Self::ContentLength(inner) => write!(f, "{}", inner),
            Self::ContentType(inner) => write!(f, "{}", inner),
            Self::Date(inner) => write!(f, "{}", inner),
            Self::ErrorInfo(inner) => write!(f, "{}", inner),
            Self::Event(inner) => write!(f, "{}", inner),
            Self::Expires(inner) => write!(f, "{}", inner),
            Self::From(inner) => write!(f, "{}", inner),
            Self::InReplyTo(inner) => write!(f, "{}", inner),
            Self::MaxForwards(inner) => write!(f, "{}", inner),
            Self::MimeVersion(inner) => write!(f, "{}", inner),
            Self::MinExpires(inner) => write!(f, "{}", inner),
            Self::Organization(inner) => write!(f, "{}", inner),
            Self::Other(key, value) => write!(f, "{}: {}", key, value),
            Self::Priority(inner) => write!(f, "{}", inner),
            Self::ProxyAuthenticate(inner) => write!(f, "{}", inner),
            Self::ProxyAuthorization(inner) => write!(f, "{}", inner),
            Self::ProxyRequire(inner) => write!(f, "{}", inner),
            Self::RecordRoute(inner) => write!(f, "{}", inner),
            Self::ReplyTo(inner) => write!(f, "{}", inner),
            Self::Require(inner) => write!(f, "{}", inner),
            Self::RetryAfter(inner) => write!(f, "{}", inner),
            Self::Route(inner) => write!(f, "{}", inner),
            Self::Server(inner) => write!(f, "{}", inner),
            Self::Subject(inner) => write!(f, "{}", inner),
            Self::SubscriptionState(inner) => write!(f, "{}", inner),
            Self::Supported(inner) => write!(f, "{}", inner),
            Self::Timestamp(inner) => write!(f, "{}", inner),
            Self::To(inner) => write!(f, "{}", inner),
            Self::Unsupported(inner) => write!(f, "{}", inner),
            Self::UserAgent(inner) => write!(f, "{}", inner),
            Self::Via(inner) => write!(f, "{}", inner),
            Self::Warning(inner) => write!(f, "{}", inner),
            Self::WwwAuthenticate(inner) => write!(f, "{}", inner),
        }
    }
}

#[doc(hidden)]
pub mod tokenizer {
    use super::*;
    use crate::{Error, IResult, TokenizerError};
    use rsip_derives::Utf8Tokenizer;
    use std::convert::TryInto;

    impl<'a> TryInto<Header> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Header, Error> {
            let tokenizer: Utf8Tokenizer = self.try_into()?;

            match tokenizer.name {
                s if s.eq_ignore_ascii_case("Accept") => {
                    Ok(Header::Accept(Accept::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Accept-Encoding") => {
                    Ok(Header::AcceptEncoding(AcceptEncoding::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Accept-Language") => {
                    Ok(Header::AcceptLanguage(AcceptLanguage::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Alert-Info") => {
                    Ok(Header::AlertInfo(AlertInfo::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Allow") => {
                    Ok(Header::Allow(Allow::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Authentication-Info") => Ok(
                    Header::AuthenticationInfo(AuthenticationInfo::new(tokenizer.value)),
                ),
                s if s.eq_ignore_ascii_case("Authorization") => {
                    Ok(Header::Authorization(Authorization::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("CSeq") => Ok(Header::CSeq(CSeq::new(tokenizer.value))),
                s if s.eq_ignore_ascii_case("Call-Id") || s.eq_ignore_ascii_case("i") => {
                    Ok(Header::CallId(CallId::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Call-Info") => {
                    Ok(Header::CallInfo(CallInfo::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Contact") || s.eq_ignore_ascii_case("m") => {
                    Ok(Header::Contact(Contact::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Content-Disposition") => Ok(
                    Header::ContentDisposition(ContentDisposition::new(tokenizer.value)),
                ),
                s if s.eq_ignore_ascii_case("Content-Encoding") || s.eq_ignore_ascii_case("e") => {
                    Ok(Header::ContentEncoding(ContentEncoding::new(
                        tokenizer.value,
                    )))
                }
                s if s.eq_ignore_ascii_case("Content-Language") => Ok(Header::ContentLanguage(
                    ContentLanguage::new(tokenizer.value),
                )),
                s if s.eq_ignore_ascii_case("Content-Length") || s.eq_ignore_ascii_case("l") => {
                    Ok(Header::ContentLength(ContentLength::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Content-Type") || s.eq_ignore_ascii_case("c") => {
                    Ok(Header::ContentType(ContentType::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Date") => Ok(Header::Date(Date::new(tokenizer.value))),
                s if s.eq_ignore_ascii_case("Error-Info") => {
                    Ok(Header::ErrorInfo(ErrorInfo::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Event") => {
                    Ok(Header::Event(Event::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Expires") => {
                    Ok(Header::Expires(Expires::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("From") || s.eq_ignore_ascii_case("f") => {
                    Ok(Header::From(From::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("In-Reply-To") => {
                    Ok(Header::InReplyTo(InReplyTo::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Max-Forwards") => {
                    Ok(Header::MaxForwards(MaxForwards::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Mime-Version") => {
                    Ok(Header::MimeVersion(MimeVersion::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Min-Expires") => {
                    Ok(Header::MinExpires(MinExpires::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Organization") => {
                    Ok(Header::Organization(Organization::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Priority") => {
                    Ok(Header::Priority(Priority::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Proxy-Authenticate") => Ok(Header::ProxyAuthenticate(
                    ProxyAuthenticate::new(tokenizer.value),
                )),
                s if s.eq_ignore_ascii_case("Proxy-Authorization") => Ok(
                    Header::ProxyAuthorization(ProxyAuthorization::new(tokenizer.value)),
                ),
                s if s.eq_ignore_ascii_case("Proxy-Require") => {
                    Ok(Header::ProxyRequire(ProxyRequire::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Record-Route") => {
                    Ok(Header::RecordRoute(RecordRoute::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Reply-To") => {
                    Ok(Header::ReplyTo(ReplyTo::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Require") => {
                    Ok(Header::Require(Require::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Retry-After") => {
                    Ok(Header::RetryAfter(RetryAfter::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Route") => {
                    Ok(Header::Route(Route::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Server") => {
                    Ok(Header::Server(Server::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Subject") || s.eq_ignore_ascii_case("s") => {
                    Ok(Header::Subject(Subject::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Supported") || s.eq_ignore_ascii_case("k") => {
                    Ok(Header::Supported(Supported::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Timestamp") => {
                    Ok(Header::Timestamp(Timestamp::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("To") || s.eq_ignore_ascii_case("t") => {
                    Ok(Header::To(To::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Unsupported") => {
                    Ok(Header::Unsupported(Unsupported::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("User-Agent") => {
                    Ok(Header::UserAgent(UserAgent::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Via") || s.eq_ignore_ascii_case("v") => {
                    Ok(Header::Via(Via::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Warning") => {
                    Ok(Header::Warning(Warning::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("WWW-Authenticate") => Ok(Header::WwwAuthenticate(
                    WwwAuthenticate::new(tokenizer.value),
                )),
                _ => Ok(Header::Other(tokenizer.name.into(), tokenizer.value.into())),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Utf8Tokenizer)]
    pub struct Tokenizer<'a> {
        pub name: &'a [u8],
        pub value: &'a [u8],
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a [u8]) -> IResult<Self> {
            use crate::NomError;
            use nom::{
                branch::alt,
                bytes::complete::{tag, take_until, take_while1},
                character::complete::space0,
                combinator::{map, rest},
                sequence::tuple,
            };

            let (rem, (name, _, _, value)) = tuple((
                take_while1(crate::parser_utils::is_token),
                tag(":"),
                space0,
                alt((
                    map(tuple((take_until("\r\n"), tag("\r\n"))), |(value, _)| value),
                    rest,
                )),
            ))(part)
            .map_err(|_: NomError<'a>| TokenizerError::from(("header", part)).into())?;

            Ok((rem, (name, value).into()))
        }
    }

    impl<'a> std::convert::From<(&'a [u8], &'a [u8])> for Tokenizer<'a> {
        fn from(tuple: (&'a [u8], &'a [u8])) -> Self {
            Self {
                name: tuple.0,
                value: tuple.1,
            }
        }
    }
}
