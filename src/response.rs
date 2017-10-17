use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io::prelude::*;

use constants::ResponseCode;
use request::Request;
use transports::Transport;

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

    pub fn parse_headers(&mut self, data: &[u8]) -> Result<(), String> {
        let header_text = String::from_utf8(data.to_vec()).unwrap();
        let mut header_lines = header_text.lines();
        if !header_text.starts_with("HTTP/1.1") {
            return Err("Invalid http response!".to_owned());
        }
        let mut header_parts = header_lines.next().unwrap().split_whitespace();
        self.response_code = ResponseCode::from_int(header_parts.nth(1).unwrap().parse::<u32>().unwrap());

        while let Some(line) = header_lines.next() {
            let mut line_parts = line.split(": ");
            self.headers.insert(line_parts.next().unwrap().to_owned(), line_parts.next().unwrap().to_owned());
        }

        Ok(()) 
    }

    /// Returns a Response object from a given stream
    pub fn from_request(stream: &mut Box<Transport>, request: Request) -> Result<Response, String> {
        let _ = stream.write(&request.to_payload());
        let mut data = Vec::new();
        let _ = stream.make_request(&mut data);

        if let Some(i) = Response::find_body_start(&data.clone()) {
            let mut response = Response::new();
            response.parse_headers(&data[0..i-2])?;
            response.set_body(&data[i+1..]);
            return Ok(response);
        }
        Err("Could not parse http response!".to_owned())    
    }

    /// Finds the end of the http header block
    fn find_body_start(data: &[u8]) -> Option<usize> {
        for i in 3..data.len() {
            if 
                data[i-3] == b'\r' &&
                data[i-2] == b'\n' &&
                data[i-1] == b'\r' &&
                data[i] == b'\n'
            {
                return Some(i);
            }
        }
        None
    }
}