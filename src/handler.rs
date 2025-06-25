use std::collections::HashMap;
use std::fs::OpenOptions;
use std::{env, fs};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::{io::Write, net::TcpStream};


use crate::lib::{HttpMethod, StatusCode};
use crate::req::HttpRequest;
use crate::res::HttpResponse;
use crate::util::get_directory;
pub fn route_handler(req: &mut HttpRequest, res: &mut HttpResponse) {
    print!("{}", req.target);
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
            let dir = get_directory();
            let file_path = &path["/files/".len()..path.len()];
            let file_path = dir.as_path().join(Path::new(file_path));
            print!("{:?}", file_path);
            // print!("whwhjhdhwj");
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
                            headers.insert("Content-Type".to_string(), "application/octet-stream".to_string());
                            res.send(Some(body.as_bytes().to_vec()), Some(headers), StatusCode::Ok);
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
                            let data = req.body.clone().concat();
                            let data = data.as_bytes();
                            let write_to = f.write_all(&data);
                            // let f = f.try_clone();
                            match write_to {
                                Ok(()) => {
                                    res.send(None, None, StatusCode::Created);
                                }
                                _ => { res.send(None, None, StatusCode::InternalServerError); }
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
            print!("not found");
            res.send(None, None, StatusCode::NotFound);
        }
    }
}
