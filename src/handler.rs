use std::collections::HashMap;
use std::{io::Write, net::TcpStream};

use crate::lib::StatusCode;
use crate::req::HttpRequest;
use crate::res::HttpResponse;
pub fn route_handler(req: &mut HttpRequest, res: &mut HttpResponse) {
    print!("{}", req.target);
    match req.target.as_str() {
        "/" => {
            res.send(None, None, StatusCode::Ok);
        }
        path if path.starts_with("/echo/") => {
            let id = &path["/echo/".len()..];
            let body = Some(Vec::from(id.to_string().as_bytes()));
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "text/plain".to_string());
            res.send(body, Some(headers), StatusCode::Ok);
        }
        _ => {
            let response = "HTTP/1.1 404 Not Found\r\n\r\n";
            req.stream.write_all(response.as_bytes()).unwrap();
        }
    }
}
