use std::{collections::HashMap, io::Write, net::TcpStream};

use crate::{
    lib::StatusCode,
    req::{self, HttpRequest},
};

pub struct HttpResponse {
    pub status_code: StatusCode,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub stream: TcpStream,
}

impl HttpResponse {
    pub fn new(
        status_code: StatusCode,
        headers: HashMap<String, String>,
        body: Option<Vec<u8>>,
        stream: TcpStream,
    ) -> Self {
        HttpResponse {
            status_code,
            http_version: String::from("HTTP/1.1"),
            headers,
            body: body,
            stream,
        }
    }

    pub fn set_header(k: String, v: String) {}

    pub fn set_body(body: Vec<String>) {}

    pub fn send(
        &mut self,
        body: Option<Vec<u8>>,
        headers: Option<HashMap<String, String>>,
        status_code: StatusCode,
    ) {
        let mut response = format!(
            "HTTP/1.1 {} {}\r\n",
            status_code.as_u16(),
            status_code.reason_phrase()
        );

        match headers {
            Some(mut r) => {
                let mut default_headers = HashMap::new();
                default_headers.insert("Content-Type".to_string(), "text/plain".to_string());
                for v in default_headers {
                    r.entry(v.0).or_insert(v.1);
                }
                for v in r {
                    response.push_str(format!("{}: {}\r\n", v.0, v.1).as_str());
                }
            }
            None => {}
        }
        // add default headers

        let body_len = body.as_ref().map(|b| b.len()).unwrap_or(0);
        // headers.insert("Content-Length".to_string(), body_len.to_string());
        response.push_str(format!("Content-Length: {}\r\n", body_len).as_str());
        response.push_str("\r\n");
        let mut response = response.as_bytes().to_vec();
        if let Some(r) = body {
            response.extend(r);
        }
        self.stream.write_all(&response).unwrap();
    }
}
