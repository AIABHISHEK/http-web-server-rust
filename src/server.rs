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
use threadpool::ThreadPool;

pub fn start_tcp() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(15);
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                pool.execute(|| handle_connection(_stream));
                // handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(stream: TcpStream) {
    // println!("hjdhahdahhawdwajd");
    let res_stream = stream.try_clone().expect("failed stream cloning");
    let mut req = parse_incoming_req(stream);
    let req_headers = req.headers.clone();
    let mut res = HttpResponse::new(StatusCode::Ok, req_headers, None, res_stream);
    route_handler(&mut req, &mut res);
}
