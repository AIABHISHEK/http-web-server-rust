use std::net::TcpListener;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use crate::handler::route_handler;
use crate::lib::StatusCode;
use crate::req;
use crate::res::HttpResponse;
use crate::util::parse_incoming_req;

pub fn start_tcp() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    // print!("1  ");
    for stream in listener.incoming() {
        // print!("2  ");
        match stream {
            Ok(_stream) => {
                handle_connection(_stream);
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(stream: TcpStream) {
    // let request_line: Vec<&str> = http_request.get(0).unwrap().split(" ").collect();
    let res_stream = stream.try_clone().expect("failed stream cloning");
    let mut req = parse_incoming_req(stream);
    let response = "HTTP/1.1 404 Not Found\r\n\r\n";
    let req_headers = req.headers.clone();
    let mut res = HttpResponse::new(StatusCode::Ok, req_headers, None, res_stream);
    route_handler(&mut req, &mut res);
}
