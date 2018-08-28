use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::prelude::*;

use constants::{ResponseCode, TransferEncoding};
use request::Request;
use transports::{find_sub_vector, Transport};

pub struct Response {
    response_code: ResponseCode,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    /// Returns a default (empty) response with response code of 0 (undefined)
    pub fn new() -> Response {
        Response {
            response_code: ResponseCode::Ok,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    /// Sets the response code
    pub fn set_response_code(&mut self, response_code: ResponseCode) {
        self.response_code = response_code;
    }

    /// Returns the response code
    pub fn response_code(&self) -> ResponseCode {
        self.response_code.clone()
    }

    /// Adds a header to the response
    pub fn add_header(&mut self, name: String, value: String) {
        self.headers.insert(name, value);
    }

    /// Returns a copy of the header HashMap
    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    /// Returns an option of a specific header
    pub fn get_header(&mut self, name: String) -> Option<String> {
        match self.headers.entry(name) {
            Entry::Occupied(e) => Some(e.get().to_owned()),
            _ => None,
        }
    }

    /// Sets the body payload
    pub fn set_body(&mut self, data: &[u8]) {
        self.body = data.to_vec();
    }

    /// Returns a copy of the body in the response
    pub fn body(&self) -> Vec<u8> {
        self.body.clone()
    }

    pub fn parse_headers(&mut self, data: &[u8]) -> Result<TransferEncoding, String> {
        let mut transfer_encoding = TransferEncoding::Normal;
        let header_text = String::from_utf8(data.to_vec()).map_err(|e| e.to_string())?;
        let mut header_lines = header_text.lines();
        if !header_text.starts_with("HTTP/1.1") {
            return Err("Invalid http response!".to_owned());
        }
        let mut header_parts = header_lines
            .next()
            .ok_or("No more header lines".to_owned())?
            .split_whitespace();
        self.response_code = ResponseCode::from_int(
            header_parts
                .nth(1)
                .ok_or("Unable to parse response code".to_owned())?
                .parse::<u32>()
                .map_err(|e| e.to_string())?,
        );

        while let Some(line) = header_lines.next() {
            let mut line_parts = line.split(": ");
            if line_parts.clone().collect::<Vec<&str>>().len() < 2 {
                continue;
            }
            if let Some(header) = line_parts.clone().nth(0) {
                if header.to_lowercase() == "transfer-encoding" {
                    if let Some(te) = line_parts.clone().nth(1) {
                        if te.to_lowercase().contains("chunked") {
                            transfer_encoding = TransferEncoding::Chunked;
                        }
                    }
                }
            }
            self.headers.insert(
                line_parts
                    .next()
                    .ok_or("Malformed header".to_owned())?
                    .to_owned(),
                line_parts
                    .next()
                    .ok_or("Malformed header".to_owned())?
                    .to_owned(),
            );
        }

        Ok(transfer_encoding)
    }

    fn parse_body(
        &mut self,
        data: &mut [u8],
        transfer_encoding: TransferEncoding,
        last_chunk_size: Option<usize>,
    ) -> Result<(), String> {
        match transfer_encoding {
            TransferEncoding::Normal => {
                self.set_body(data);
                Ok(())
            }
            TransferEncoding::Chunked => {
                if data.to_vec() == "0\r\n\r\n".as_bytes().to_vec() {
                    return Ok(());
                }
                let first_look = usize::min(8, data.len());
                if let Some(offset) =
                    find_sub_vector(&data[0..first_look].to_vec(), &"\r\n".as_bytes().to_vec())
                {
                    let chunk_len = usize::from_str_radix(
                        &String::from_utf8(data[0..offset - 2].to_vec()).map_err(|e| {
                            format!("Error parsing chunked encoding: {}", e.to_string())
                        })?,
                        16,
                    ).map_err(|e| {
                        format!("Error parsing chunked encoding: {}", e.to_string())
                    })?;
                    self.body
                        .append(&mut data[offset..offset + chunk_len].to_vec());
                    self.parse_body(
                        &mut data[offset + chunk_len + 2..],
                        transfer_encoding,
                        Some(last_chunk_size.unwrap_or(0) + chunk_len),
                    )
                } else {
                    Err(format!(
                        "Error parsing chunked data: unable to read chunk size"
                    ))
                }
            }
        }
    }

    /// Returns a Response object from a given stream
    pub fn from_request(stream: &mut Box<Transport>, request: Request) -> Result<Response, String> {
        let _ = stream.write(&request.to_payload());
        let mut data = Vec::new();
        let _ = stream.make_request(&mut data);

        if let Some(i) = Response::find_body_start(&data.clone()) {
            let mut response = Response::new();
            let transfer_encoding = response.parse_headers(&data[0..i - 2])?;
            response.parse_body(&mut data[i + 1..], transfer_encoding, None)?;
            return Ok(response);
        }
        Err("Could not parse http response!".to_owned())
    }

    /// Finds the end of the http header block
    fn find_body_start(data: &[u8]) -> Option<usize> {
        for i in 3..data.len() {
            if data[i - 3] == b'\r'
                && data[i - 2] == b'\n'
                && data[i - 1] == b'\r'
                && data[i] == b'\n'
            {
                return Some(i);
            }
        }
        None
    }
}
