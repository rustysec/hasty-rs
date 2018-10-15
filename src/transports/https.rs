extern crate webpki;

use std::io::{Error as IoError, Read, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::Arc;
use std::time::Duration;

use config::Config;
use rustls::{
    Certificate, ClientConfig, ClientSession, RootCertStore, ServerCertVerified,
    ServerCertVerifier, Session, TLSError,
};
use transports::{find_sub_vector, Transport};
use url::Host;

pub struct UnsafeCertVerifier {}

impl ServerCertVerifier for UnsafeCertVerifier {
    fn verify_server_cert(
        &self,
        _: &RootCertStore,
        _: &[Certificate],
        _: &str,
        //        _: webpki::DNSNameRef,
        _: &[u8],
    ) -> Result<ServerCertVerified, TLSError> {
        Ok(ServerCertVerified::assertion())
    }
}

pub struct HttpsTransport {
    socket: TcpStream,
    session: ClientSession,
}

impl HttpsTransport {
    pub fn new(
        host: Option<Host<&str>>,
        port: Option<u16>,
        config: &Config,
    ) -> Result<Box<Transport>, String> {
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
            tls.dangerous()
                .set_certificate_verifier(Arc::new(UnsafeCertVerifier {}));
        }

        /* TODO: support requests by IP
        match webpki::DNSNameRef::try_from_ascii_str(&host) {
            Ok(host_dnsname) => {
                let t = HttpsTransport {
                    socket: TcpStream::connect(format!("{}:{:?}", host, port)).map_err(|e| e.to_string())?,
                    session: ClientSession::new(&Arc::new(tls), host_dnsname)
                };
                Ok(Box::new(Transport::Https(t)))
            }
            Err(()) => Err(format!("Invalid hostname: {}", host))
        }
        */

        let t = HttpsTransport {
            socket: TcpStream::connect(format!("{}:{:?}", host, port))
                .map_err(|e| e.to_string())?,
            session: ClientSession::new(&Arc::new(tls), &host),
        };
        Ok(Box::new(Transport::Https(t)))
    }

    pub fn make_request(&mut self, data: &mut Vec<u8>) -> Result<usize, IoError> {
        let mut chunked_encoding = false;
        loop {
            if self.session.wants_read() && self.socket_is_ready_to_read() {
                match self.session.read_tls(&mut self.socket) {
                    Ok(_) => {}
                    Err(e) => warn!("Error reading TLS stream: {}", e.to_string()),
                }
                match self.session.process_new_packets() {
                    Ok(_) => {}
                    Err(e) => warn!("Error processing TLS packets: {}", e.to_string()),
                }

                let mut tmp_data = Vec::new();
                match self.session.read_to_end(&mut tmp_data) {
                    Ok(recv_bytes) => {
                        if recv_bytes == 0 {
                            continue;
                        }

                        if chunked_encoding {
                            if tmp_data.ends_with("0\r\n\r\n".as_bytes()) {
                                data.append(&mut tmp_data);
                                break;
                            } else {
                                data.append(&mut tmp_data);
                            }
                        } else {
                            if let Some(header_length) =
                                find_sub_vector(&tmp_data, &"\r\n\r\n".as_bytes().to_vec())
                            {
                                match String::from_utf8(tmp_data[0..header_length].to_vec()) {
                                    Ok(h) => {
                                        // TODO: make this smarter
                                        if h.to_lowercase().contains("transfer-encoding: chunked") {
                                            chunked_encoding = true;
                                        }
                                        data.append(&mut tmp_data);
                                        if !chunked_encoding {
                                            break;
                                        }
                                    }
                                    Err(e) => {
                                        data.append(&mut tmp_data);
                                        trace!("Can't parse headers {}", e.to_string());
                                        break;
                                    }
                                }
                            } else {
                                trace!("Cannot find headers in response!");
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        /* handle this */
                        warn!("Error: {}", e.to_string());
                        break;
                    }
                }
            }
            if self.session.wants_write() {
                match self.session.write_tls(&mut self.socket) {
                    Ok(_) => {}
                    Err(e) => {
                        warn!("Error writing to TLS stream: {}", e.to_string());
                        break;
                    }
                }
            }
        }
        match self.socket.shutdown(Shutdown::Both) {
            Ok(_) => {}
            Err(e) => warn!("Error shutting down TCP connection: {}", e.to_string()),
        }
        Ok(data.len())
    }

    pub fn socket_is_ready_to_read(&mut self) -> bool {
        self.socket
            .set_read_timeout(Some(Duration::new(0, 100)))
            .unwrap();
        let mut tmp = vec![1];
        match self.socket.peek(&mut tmp) {
            Ok(0) => false,
            Ok(_) => true,
            Err(_) => false,
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
