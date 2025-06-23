#[allow(unused_imports)]
use std::net::TcpListener;
use std::{io::{BufRead, BufReader, Write}, net::TcpStream};

// use anyhow::{Error, Ok};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                // println!("accepted new connection");
                handle_connection(_stream);
                // _stream.
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {http_request:#?}");
    let request_line: Vec<&str> = http_request.get(0).unwrap().split(" ").collect();
    if request_line[1] == "/" {
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
    else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }

}
