pub mod http;
pub mod https;

extern crate rustls;

use std::io::{Error as IoError, Read, Result as IoResult, Write};

pub use http::HttpTransport;
pub use https::HttpsTransport;

//pub enum Transport<'a, T: 'a + Read + Write> {
pub enum Transport {
    Https(HttpsTransport),
    Http(HttpTransport),
}

impl Transport {
    pub fn make_request(&mut self, data: &mut Vec<u8>) -> Result<usize, IoError> {
        match *self {
            Transport::Http(ref mut t) => t.make_request(data),
            Transport::Https(ref mut t) => t.make_request(data),
        }
    }
}

impl Read for Transport {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        match *self {
            Transport::Http(ref mut t) => t.read(buf),
            Transport::Https(ref mut t) => t.read(buf),
        }
    }
}

impl Write for Transport {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        match *self {
            Transport::Http(ref mut t) => t.write(buf),
            Transport::Https(ref mut t) => t.write(buf),
        }
    }

    fn flush(&mut self) -> IoResult<()> {
        match *self {
            Transport::Http(ref mut t) => t.flush(),
            Transport::Https(ref mut t) => t.flush(),
        }
    }
}

pub fn find_sub_vector(large: &Vec<u8>, small: &Vec<u8>) -> Option<usize> {
    if large.len() < small.len() {
        return None;
    } else {
        let mut i = 0;
        while i < (large.len() - small.len()) {
            let tmp: Vec<u8> = large
                .iter()
                .skip(i)
                .take(small.len())
                .map(|&i| i.to_owned())
                .collect();

            if &tmp == small {
                return Some(i + small.len());
            }
            i += 1;
        }
    }
    None
}
