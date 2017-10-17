pub mod http;
pub mod https;

extern crate rustls;

use std::io::{Read, Write, Result as IoResult, Error as IoError};

pub use http::HttpTransport;
pub use https::HttpsTransport;

//pub enum Transport<'a, T: 'a + Read + Write> {
pub enum Transport {
    Https(HttpsTransport),
    Http(HttpTransport)
}

impl Transport {
    pub fn make_request(&mut self, data: &mut Vec<u8>) -> Result<usize, IoError> {
        match *self {
            Transport::Http(ref mut t) => t.make_request(data),
            Transport::Https(ref mut t) => t.make_request(data)
        }
    }
}

impl Read for Transport {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        match *self {
            Transport::Http(ref mut t) => t.read(buf),
            Transport::Https(ref mut t) => t.read(buf)
        }
    }
}

impl Write for Transport {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        match *self {
            Transport::Http(ref mut t) => t.write(buf),
            Transport::Https(ref mut t) => t.write(buf)
        }
    }

    fn flush(&mut self) -> IoResult<()> {
        match *self {
            Transport::Http(ref mut t) => t.flush(),
            Transport::Https(ref mut t) => t.flush()
        }
    }
}
