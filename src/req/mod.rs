use std::{collections::HashMap, net::TcpStream};

use crate::lib::HttpMethod;

pub struct HttpRequest {
    pub method: HttpMethod,
    pub target: String,   //http target
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<String>,
    pub stream: TcpStream,
}

impl HttpRequest {
    pub fn new(method:HttpMethod, target:String, http_version: String, header: HashMap<String, String>, body: Vec<String>, stream: TcpStream) -> HttpRequest {
        HttpRequest {
            method: method,
            target: target,
            http_version: http_version,
            headers: header,
            body: body,
            stream: stream,
        }
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        self.headers.get(key).map(|v| v.to_string())
    }
}
