use std::collections::HashMap;
use std::io::Write;
use std::io::{BufReader, Read};
use std::path;
use std::{env, fs};

use crate::lib::{HttpMethod, StatusCode};
use crate::req::HttpRequest;
use crate::res::HttpResponse;
use crate::util::get_directory;
pub fn route_handler(req: &mut HttpRequest, res: &mut HttpResponse) {
    // println!("{}", req.target);
    let mut headers = HashMap::new();
    if req.headers.get("Accept-Encoding") == Some(&"gzip".to_string()) {
        headers.insert("Content-Encoding".to_string(), "gzip".to_string());
    }
    match req.headers.get("Accept-Encoding") {
        Some(r) => {
            let encode_accepted = r.split(", ").into_iter().any(|v| v == "gzip");
            if encode_accepted {
                headers.insert("Content-Encoding".to_string(), "gzip".to_string());
            }
        }
        None => {}
    }
    if req.headers.contains_key("Connection") {
        headers.insert("Connection".to_string(), req.headers.get("Connection").unwrap().to_string());
    }
    match req.target.as_str() {
        "/" => {
            res.send(None, None, StatusCode::Ok);
        }
        path if path.starts_with("/echo/") => {
            let id = &path["/echo/".len()..];
            let body = Some(Vec::from(id.to_string().as_bytes()));

            res.send(body, Some(headers), StatusCode::Ok);
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
            let mut dir = get_directory();
            let file_path = &path["/files/".len()..path.len()];
            let file_path = dir.join(file_path);
            // println!("{:?}", file_path);
            match req.method {
                HttpMethod::GET => {
                    let file = fs::File::open(file_path);
                    match file {
                        Ok(f) => {
                            let mut reader = BufReader::new(f);
                            let mut body = String::new();
                            if let Err(e) = reader.read_to_string(&mut body) {
                                res.send(None, None, StatusCode::NotFound);
                            }
                            let mut headers: HashMap<String, String> = HashMap::new();
                            headers.insert(
                                "Content-Type".to_string(),
                                "application/octet-stream".to_string(),
                            );
                            res.send(
                                Some(body.as_bytes().to_vec()),
                                Some(headers),
                                StatusCode::Ok,
                            );
                        }
                        _ => {
                            res.send(None, None, StatusCode::NotFound);
                        }
                    }
                }
                HttpMethod::POST => {
                    let file = fs::File::create(file_path);
                    match file {
                        Ok(mut f) => {
                            let data = &req.body;
                            let write_to = f.write_all(data);
                            match write_to {
                                Ok(()) => {
                                    res.send(None, None, StatusCode::Created);
                                }
                                _ => {
                                    res.send(None, None, StatusCode::InternalServerError);
                                }
                            }
                        }
                        _ => {
                            res.send(None, None, StatusCode::NotFound);
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {
            res.send(None, None, StatusCode::NotFound);
        }
    }
}
