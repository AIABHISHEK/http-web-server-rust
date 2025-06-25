use std::{
    collections::HashMap,
    env,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
    path::PathBuf,
};

use crate::{lib::HttpMethod, req::HttpRequest};

pub fn parse_incoming_req(mut stream: TcpStream) -> HttpRequest {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = Vec::new();
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut body: Vec<String> = Vec::new();
    let mut is_header_complete = false;
    for line in buf_reader.by_ref().lines() {
        match line {
            Ok(r) => {
                if r.is_empty() {
                    is_header_complete = true;
                    break;
                }
                if request_line.is_empty() {
                    request_line = r.split(' ').map(|v| v.to_string()).collect();
                } else if !is_header_complete {
                    let v = r
                        .split_once(": ")
                        .map(|val| (val.0.to_string(), val.1.to_string()));
                    if let Some(r) = v {
                        headers.insert(r.0, r.1);
                    }
                }
                // else {
                //     if let Some(cl) = headers.get("Content-Length") {
                //         body.push(r);
                //     }
                //     body.push(r);
                // }
            }
            Err(e) => {}
        }
    }

    if let Some(cl) = headers.get("Content-Length") {
        if let Ok(len) = cl.parse::<usize>() {
            let mut buf = vec![0; len];
            buf_reader.read_exact(&mut buf).unwrap();
            // let b:Vec<char> = buf.iter().map(|&b| b.clone() as char).collect();
        }
    }
    println!("end of parsing");

    let method = HttpMethod::from_string(&request_line[0]).unwrap_or(HttpMethod::GET);
    let target = request_line[1].clone();
    let http_version = request_line[2].clone();
    let req = HttpRequest::new(method, target, http_version, headers, body, stream);
    // return true;
    return req;
}

pub fn build_response() {}

pub fn get_directory() -> PathBuf {
    let mut args = env::args();
    let mut directory = None;
    while let Some(arg) = args.next() {
        if arg == "--directory" {
            if let Some(path_str) = args.next() {
                directory = Some(PathBuf::from(path_str));
            } else {
                eprintln!("Error: --directory requires a path");
                std::process::exit(1);
            }
            break;
        }
    }
    // let base_dir = Arc
    let dir = directory.expect("Missing --directory argument");
    return dir;
}
