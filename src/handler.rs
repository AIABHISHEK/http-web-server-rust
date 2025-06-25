use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};
use std::{io::Write, net::TcpStream};

use crate::lib::StatusCode;
use crate::req::HttpRequest;
use crate::res::HttpResponse;
pub fn route_handler(req: &mut HttpRequest, res: &mut HttpResponse) {
    // print!("{}", req.target);
    match req.target.as_str() {
        "/" => {
            res.send(None, None, StatusCode::Ok);
        }
        path if path.starts_with("/echo/") => {
            let id = &path["/echo/".len()..];
            let body = Some(Vec::from(id.to_string().as_bytes()));
            res.send(body, None, StatusCode::Ok);
        }
        "/user-agent" => {
            let user_agent = req.get_header("User-Agent");
            match user_agent {
                Some(r) => {
                    let body = Some(Vec::from(r.as_bytes()));
                    res.send(body, None, StatusCode::Ok);
                }
                None => {
                    res.send(None, None, StatusCode::NotFound);
                }
            }
        }
        path if path.starts_with("/files/") => {
            let file_path = &path["/files/".len()..path.len()];
            let file = fs::File::open(file_path);
            match file {
                Ok(f) => {
                    let mut reader = BufReader::new(f);
                    let mut body = String::new();
                    if let Err(e) = reader.read_to_string(&mut body) {
                        res.send(None, None, StatusCode::NotFound);
                    }
                    res.send(Some(body.as_bytes().to_vec()), None, StatusCode::Ok);
                }
                _ => { res.send(None, None, StatusCode::NotFound); }
            }
        }
        _ => {
            res.send(None, None, StatusCode::NotFound);
        }
    }
}
