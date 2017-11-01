extern crate mime;

use std::collections::HashMap;

use url::{Host,Url};

use constants::HttpMethods;

#[derive(Clone)]
pub struct Request {
    host: String,
    path: String,
    method: HttpMethods,
    headers: HashMap<String,String>,
    body: Option<Vec<u8>>,
    body_type: mime::Mime,
    url: Option<Url>,
}

impl Request {
    /// Returns a default (simple GET) request
    pub fn new() -> Request {
        Request {
            host: "".to_string(),
            path: "/".to_string(),
            method: HttpMethods::Get,
            headers: HashMap::new(),
            body: None,
            body_type: mime::TEXT_PLAIN,
            url: None
        }
    }

    /// Returns a default get for a given Url identifier
    ///
    /// # Example
    ///
    /// ```
    /// extern crate hasty;
    /// extern crate url;
    ///
    /// use hasty::Request;
    /// use url::Url;
    ///
    /// fn main() {
    ///     let url = url::Url::parse("http://www.rust-lang.org").unwrap();
    ///     let req = Request::from_url(url);
    /// }
    /// ```
    pub fn from_url(url: Url) -> Request {
        Request {
            host: url.host().unwrap_or(Host::Domain("www.rust-lang.org")).to_string(),
            path: url.path().to_owned(),
            method: HttpMethods::Get,
            headers: HashMap::new(),
            body: None,
            body_type: mime::TEXT_PLAIN,
            url: Some(url),
        }
    }

    /// Set the host and path based on provided url
    pub fn with_url(&mut self, url: Url) {
        self.host = url.host().unwrap_or(Host::Domain("www.rust-lang.org")).to_string();
        self.path = url.path().to_owned();
        self.url = Some(url);
    }

    /// Get the URL
    pub fn url(&self) -> Option<Url> {
        self.url.clone()
    }

    /// Set the HTTP method
    pub fn with_method(&mut self, method: HttpMethods) {
        self.method = method;
    }

    /// Set the content type for the reqeust body
    pub fn with_content_type(&mut self, content_type: mime::Mime) {
        self.body_type = content_type;
    }

    /// Set the payload of the request
    pub fn with_body(&mut self, body: Option<Vec<u8>>) {
        self.body = body;
    }

    /// Adds a user defined header to the request
    ///
    /// # Example
    /// ```
    /// use hasty::Request;
    /// let req = Request::new().add_raw_header("Authentication".to_owned(), "MyApiKey".to_owned());
    /// ```
    pub fn add_raw_header(mut self, name: String, value: String) -> Self {
        self.headers.insert(name, value);
        self
    }

    /// Returns the request as a transmittable payload
    ///
    /// # Example
    ///
    /// ```
    /// extern crate hasty;
    /// extern crate url;
    ///
    /// use hasty::Request;
    /// use url::Url;
    ///
    /// fn main() {
    ///     let url = url::Url::parse("http://www.rust-lang.org").unwrap();
    ///     let req = Request::from_url(url);
    ///     let payload = String::from_utf8(req.to_payload()).unwrap();
    ///     assert_eq!(&payload, "GET / HTTP/1.1\r\nHost: www.rust-lang.org\r\n\r\n");
    /// }
    /// ```
    pub fn to_payload(self) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend(
            format!(
                "{} {} HTTP/1.1\r\n",
                self.method.to_string(), self.path
            ).as_bytes()
        );
        payload.extend(
            format!(
                "Host: {}\r\n", self.host
            ).as_bytes()
        );
        for header in self.headers {
            payload.extend(
                format!(
                    "{}: {}\r\n", header.0, header.1
                ).as_bytes()
            );
        }
        if let Some(content) = self.body {
            payload.extend(
                format!(
                    "content-length: {}\r\ncontent-type: {};charset=UTF-8\r\n\r\n",
                    content.len(), self.body_type
                ).as_bytes()
            );
            payload.extend(content);
        }
        payload.extend(
            "\r\n".to_owned().into_bytes()
        );
        payload
    }
}