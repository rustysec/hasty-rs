use std::fmt;

#[derive(Clone,Debug)]
pub enum HttpMethods {
    Get,
    Post,
}

impl fmt::Display for HttpMethods {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = format!("{:?}", self);
        write!(f, "{}", s.to_uppercase())
    }
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum ResponseCode {
    // Testing
    Undefined = 0,
    // Informational
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    // Success
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ImUsed = 226,
    // Redirection
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    SwitchProxy = 306,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    // Client Error
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisified = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntitiy = 422,
    Locked = 423,
    FailedDependency = 424,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
    // Server Error
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508 ,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = format!("{:?}", self).to_owned();
        write!(f, "{}", &s.to_uppercase())
    }
}

impl ResponseCode {
    pub fn from_int(i: u32) -> ResponseCode {
        match i {
            // Testing
            0 => ResponseCode::Undefined,
            // Informational
            100 => ResponseCode::Continue,
            101 => ResponseCode::SwitchingProtocols,
            102 => ResponseCode::Processing,
            // Success
            200 => ResponseCode::Ok,
            201 => ResponseCode::Created,
            202 => ResponseCode::Accepted,
            203 => ResponseCode::NonAuthoritativeInformation,
            204 => ResponseCode::NoContent,
            205 => ResponseCode::ResetContent,
            206 => ResponseCode::PartialContent,
            207 => ResponseCode::MultiStatus,
            208 => ResponseCode::AlreadyReported,
            226 => ResponseCode::ImUsed,
            // Redirection
            300 => ResponseCode::MultipleChoices,
            301 => ResponseCode::MovedPermanently,
            302 => ResponseCode::Found,
            303 => ResponseCode::SeeOther,
            304 => ResponseCode::NotModified,
            305 => ResponseCode::UseProxy,
            306 => ResponseCode::SwitchProxy,
            307 => ResponseCode::TemporaryRedirect,
            308 => ResponseCode::PermanentRedirect,
            // Client Error
            400 => ResponseCode::BadRequest,
            401 => ResponseCode::Unauthorized,
            402 => ResponseCode::PaymentRequired,
            403 => ResponseCode::Forbidden,
            404 => ResponseCode::NotFound,
            405 => ResponseCode::MethodNotAllowed,
            406 => ResponseCode::NotAcceptable,
            407 => ResponseCode::ProxyAuthenticationRequired,
            408 => ResponseCode::RequestTimeout,
            409 => ResponseCode::Conflict,
            410 => ResponseCode::Gone,
            411 => ResponseCode::LengthRequired,
            412 => ResponseCode::PreconditionFailed,
            413 => ResponseCode::PayloadTooLarge,
            414 => ResponseCode::UriTooLong,
            415 => ResponseCode::UnsupportedMediaType,
            416 => ResponseCode::RangeNotSatisified,
            417 => ResponseCode::ExpectationFailed,
            418 => ResponseCode::ImATeapot,
            421 => ResponseCode::MisdirectedRequest,
            422 => ResponseCode::UnprocessableEntitiy,
            423 => ResponseCode::Locked,
            424 => ResponseCode::FailedDependency,
            426 => ResponseCode::UpgradeRequired,
            428 => ResponseCode::PreconditionRequired,
            429 => ResponseCode::TooManyRequests,
            431 => ResponseCode::RequestHeaderFieldsTooLarge,
            451 => ResponseCode::UnavailableForLegalReasons,
            // Server Error
            500 => ResponseCode::InternalServerError,
            501 => ResponseCode::NotImplemented,
            502 => ResponseCode::BadGateway,
            503 => ResponseCode::ServiceUnavailable,
            504 => ResponseCode::GatewayTimeout,
            505 => ResponseCode::HttpVersionNotSupported,
            506 => ResponseCode::VariantAlsoNegotiates,
            507 => ResponseCode::InsufficientStorage,
            508 => ResponseCode::LoopDetected,
            510 => ResponseCode::NotExtended,
            511 => ResponseCode::NetworkAuthenticationRequired,
            _ => ResponseCode::Undefined,
        }
    }
    
}

#[test]
pub fn methods_post() {
    let s = HttpMethods::Post.to_string();
    assert_eq!(s, "POST".to_owned());
}

#[test]
pub fn methods_get() {
    let s = HttpMethods::Get.to_string();
    assert_eq!(s, "GET".to_owned());
}

#[test]
pub fn response_code_success() {
    let s = ResponseCode::Continue.to_string();
    assert_eq!(s, "CONTINUE");
}
