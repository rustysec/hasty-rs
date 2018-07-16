mod constants;
mod config;
mod request;
mod response;
mod transports;

#[macro_use] extern crate log;
extern crate url;
extern crate rustls;
#[cfg(test)] extern crate serde_json;

use url::Url;

pub use config::Config;
pub use request::Request;
pub use constants::{HttpMethods,ResponseCode};

use response::Response;
use transports::*;

pub struct Hasty {
    config: Config,
}

impl Hasty {

    /// Returns a default Hasty instance
    pub fn new() -> Hasty {
        Hasty {
            config: Config::new(),
        }
    }

    /// Returns a Hasty instance with user defined configuration
    ///
    /// # Example
    ///
    /// ```
    /// use hasty::{Config, Hasty};
    /// let config = Config::new().disable_https_security();
    /// let hasty = Hasty::new_with_config(config);
    /// ```
    pub fn new_with_config(config: Config) -> Hasty {
        Hasty {
            config: config,
        }
    }

    /// Perform an http request and return a response
    ///
    /// # Example
    ///
    /// ```
    /// extern crate hasty;
    /// extern crate url;
    /// extern crate serde_json;
    ///
    /// fn main() {
    ///     use serde_json::{Value, Error};
    ///     use url::Url;
    ///     use hasty::{Hasty,Config,Request};
    ///     let config = Config::new().disable_https_security();
    ///     let mut request = Request::new();
    ///     request.set_url("https://localhost:3001/basic_get".parse().unwrap());
    ///
    ///     let mut hasty = Hasty::new_with_config(config);
    ///     let response = hasty.request(request).unwrap();
    ///     let body = String::from_utf8(response.body()).unwrap();
    ///     assert_eq!(&body, "success");
    /// }
    /// ```
    pub fn request(&mut self, request: Request) -> Result<Response, String> {
        match request.url() {
            Some(url) => {
                let mut transport = match url.scheme() {
                    "https" => HttpsTransport::new(url.host(), url.port_or_known_default(), &self.config),
                    "http" => HttpTransport::new(url.host(), url.port_or_known_default(), &self.config),
                    _ => Err("Unsupported protocol!".to_owned()),
                }?;

                let mut req = request.clone();
                req.set_url(url);
                Response::from_request(&mut transport, req)
            }
            None => Err("No URL provided".to_owned())
        }
    }

    /// Perform an http get and return a response
    ///
    /// # Example
    ///
    /// ```
    /// extern crate hasty;
    /// extern crate url;
    /// extern crate serde_json;
    ///
    /// fn main() {
    ///     use serde_json::{Value, Error};
    ///     use url::Url;
    ///     use hasty::{Hasty,Config,Request};
    ///     let config = Config::new().disable_https_security();
    ///
    ///     let mut hasty = Hasty::new_with_config(config);
    ///     let response = hasty.get("https://localhost:3001/basic_get").unwrap();
    ///     let body = String::from_utf8(response.body()).unwrap();
    ///     assert_eq!(&body, "success");
    /// }
    /// ```
    pub fn get(&mut self, url: &str) -> Result<Response, String> {
        let url: Url = url.parse().map_err(|_| "Unable to parse url".to_owned() )?;
        let mut transport = match url.scheme() {
            "https" => HttpsTransport::new(url.host(), url.port_or_known_default(), &self.config),
            "http" => HttpTransport::new(url.host(), url.port_or_known_default(), &self.config),
            _ => Err("Unsupported protocol!".to_owned()),
        }?;

        let mut req = Request::new();
        req.set_url(url);
        Response::from_request(&mut transport, req)
    }
}

#[test]
fn http_post_basic() {
    extern crate url;
    extern crate serde_json;

    use serde_json::{Value};
    use Request;
    use Config;

    let config = Config::new().disable_https_security();
    let mut request = Request::new();
    request.set_url("https://localhost:3001/basic_post".parse().unwrap());
    request.set_method(HttpMethods::Post);
    request.add_raw_header("my-header".to_owned(), "looking good!".to_owned());

    let mut hasty = Hasty::new_with_config(config);
    let response = hasty.request(request).unwrap();
    let body = String::from_utf8(response.body()).unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["headers"]["my-header"], "looking good!");
}

#[test]
fn http_post_with_body() {
    extern crate url;
    extern crate serde_json;
    extern crate mime;

    use serde_json::{Value};

    use Request;
    use Config;

    let config = Config::new().disable_https_security();
    let mut request = Request::new();
    request.set_url("https://localhost:3001/basic_post".parse().unwrap());
    request.set_method(HttpMethods::Post);
    request.set_body(Some(vec![1,2,3,4]));
    request.set_content_type(mime::APPLICATION_OCTET_STREAM);

    let mut hasty = Hasty::new_with_config(config);
    let response = hasty.request(request).unwrap();
    let body = String::from_utf8(response.body()).unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["bodyLength"].as_f64(), Some(4.0));
}

#[test]
fn http_post_by_ip_with_body() {
    extern crate url;
    extern crate serde_json;
    extern crate mime;

    use serde_json::{Value};

    use Request;
    use Config;

    let config = Config::new().disable_https_security();
    let mut request = Request::new();
    request.set_url("https://127.0.0.1:3001/basic_post".parse().unwrap());
    request.set_method(HttpMethods::Post);
    request.set_body(Some(vec![1,2,3,4]));
    request.set_content_type(mime::APPLICATION_OCTET_STREAM);

    let mut hasty = Hasty::new_with_config(config);
    let response = hasty.request(request).unwrap();
    let body = String::from_utf8(response.body()).unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["bodyLength"].as_f64(), Some(4.0));
}
