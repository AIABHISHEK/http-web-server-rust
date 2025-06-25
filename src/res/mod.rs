use std::{collections::HashMap, io::Write, net::TcpStream};

// use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

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
        let mut compress_body = false;
        match headers {
            Some(mut r) => {
                if r.contains_key("Content-Encoding")
                    && r.get(&"Content-Encoding".to_string()) == Some(&"gzip".to_string())
                {
                    compress_body = true;
                }
                let mut default_headers = HashMap::new();
                default_headers.insert("Content-Type".to_string(), "text/plain".to_string());
                for v in default_headers {
                    r.entry(v.0).or_insert(v.1);
                }
                for v in r {
                    response.push_str(format!("{}: {}\r\n", v.0, v.1).as_str());
                }
            }
            None => {
                let mut default_headers = HashMap::new();
                default_headers.insert("Content-Type".to_string(), "text/plain".to_string());
                for v in default_headers {
                    response.push_str(format!("{}: {}\r\n", v.0, v.1).as_str());
                }
            }
        }

        let mut res_body = Vec::new();
        if !body.is_none() && compress_body {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(body.as_ref().unwrap()).unwrap();
            if let Ok( compressed) = encoder.finish() {
                res_body = compressed;
            } else {
                res_body = body.unwrap();
            }
        }
        else {
            res_body = if let Some(b) = body { b } else { Vec::new() }
        }

        let body_len = res_body.len();
        response.push_str(format!("Content-Length: {}\r\n", body_len).as_str());
        response.push_str("\r\n");
        let mut response = response.as_bytes().to_vec();
        if !res_body.is_empty() {
            response.extend(res_body);
        }
        self.stream.write_all(&response).unwrap();
    }
}
