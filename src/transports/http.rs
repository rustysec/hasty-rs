use std::io::{Read, Write, Result as IoResult, Error as IoError};
use std::net::TcpStream;

use transports::Transport;
use url::Host;
use config::Config;

pub struct HttpTransport {
    stream: TcpStream,
}

impl HttpTransport {
    pub fn new(host: Option<Host<&str>>, port: Option<u16>, _: &Config) -> Result<Box<Transport>, String> {
        if let None = host {
            return Err("Invalid host!".to_owned());
        }

        let port = match port {
            Some(p) => p,
            None => 80,
        };

        Ok(Box::new(Transport::Http(HttpTransport {
            stream: TcpStream::connect(format!("{}:{}", host.unwrap(), port)).map_err(|e| e.to_string())?,
        })))
    }

    pub fn make_request(&mut self, data: &mut Vec<u8>) -> Result<usize, IoError> {
        self.stream.read_to_end(data)
    }
}

impl Read for HttpTransport {
    fn read(&mut self, mut buf: &mut [u8]) -> IoResult<usize> {
        self.stream.read(&mut buf)
    }
}

impl Write for HttpTransport {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.stream.flush()
    }
}