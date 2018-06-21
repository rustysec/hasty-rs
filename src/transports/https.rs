extern crate webpki;

use std::io::{Read, Write, Error as IoError};
use std::net::{TcpStream, Shutdown};
use std::sync::Arc;
use std::time::Duration;

use config::Config;
use transports::Transport;
use url::Host;
use rustls::{ClientConfig, ClientSession, ServerCertVerifier, Certificate, ServerCertVerified, RootCertStore, Session, TLSError};

pub struct UnsafeCertVerifier {}

impl ServerCertVerifier for UnsafeCertVerifier {
    fn verify_server_cert(&self,
        _: &RootCertStore,
        _: &[Certificate],
//        _: &str,
        _: webpki::DNSNameRef,
        _: &[u8]) -> Result<ServerCertVerified, TLSError> {
            Ok(ServerCertVerified::assertion())
    }
}

pub struct HttpsTransport {
    socket: TcpStream,
    session: ClientSession,
}

impl HttpsTransport {
    pub fn new(host: Option<Host<&str>>, port: Option<u16>, config: &Config) -> Result<Box<Transport>, String> {
        if let None = host {
            return Err("Invalid host!".to_owned());
        }
        let host = host.unwrap().to_string();
        
        let port = match port {
            Some(p) => p,
            None => 443,
        };

        let mut tls = ClientConfig::new();
        if config.allows_insecure_https() {
            tls.dangerous().set_certificate_verifier(Arc::new(UnsafeCertVerifier {}));
        }

        match webpki::DNSNameRef::try_from_ascii_str(&host) {
            Ok(host_dnsname) => {
                let t = HttpsTransport {
                    socket: TcpStream::connect(format!("{}:{:?}", host, port)).map_err(|e| e.to_string())?,
                    session: ClientSession::new(&Arc::new(tls), host_dnsname)
                };
                Ok(Box::new(Transport::Https(t)))
            }
            Err(_) => Err(format!("Invalid hostname: {}", host))
        }
    }

    pub fn make_request(&mut self, data: &mut Vec<u8>) -> Result<usize, IoError> {
        loop {
            if self.session.wants_read() && self.socket_is_ready_to_read() {
                match self.session.read_tls(&mut self.socket) {
                    Ok(_) => {},
                    Err(e) => warn!("Error reading TLS stream: {}", e.to_string())
                }
                match self.session.process_new_packets() {
                    Ok(_) => {},
                    Err(e) => println!("Error processing TLS packets: {}", e.to_string())
                }

                match self.session.read_to_end(data) {
                    Ok(_) => {
                        match String::from_utf8(data.clone()) {
                            Ok(s) => {
                                // this is ghetto
                                if s.starts_with("HTTP") {
                                    break;
                                }
                            },
                            Err(_) => { /* handle this */ }
                        }
                    },
                    Err(_) => { /* handle this */ }
                }
            }
            if self.session.wants_write() {
                match self.session.write_tls(&mut self.socket) {
                    Ok(_) => {},
                    Err(e) => println!("Error writing to TLS stream: {}", e.to_string())
                }
            }
        }
        match self.socket.shutdown(Shutdown::Both) {
            Ok(_) => {},
            Err(e) => println!("Error shutting down TCP connection: {}", e.to_string())
        }
        Ok(data.len())
    }

    pub fn socket_is_ready_to_read(&mut self) -> bool {
        self.socket.set_read_timeout(Some(Duration::new(0, 100))).unwrap();
        let mut tmp = vec![1];
        match self.socket.peek(&mut tmp) {
            Ok(0) => false,
            Ok(_) => true,
            Err(_) => {
                false
            }
        }
    }
}

impl Read for HttpsTransport {
    fn read(&mut self, data: &mut [u8]) -> Result<usize, IoError> {
        self.session.read(data)
    }
}

impl Write for HttpsTransport {
    fn write(&mut self, data: &[u8]) -> Result<usize, IoError> {
        self.session.write(data)
    }

    fn flush(&mut self) -> Result<(), IoError> {
        self.session.flush()
    }
}
