#[allow(unused_imports)]
use std::net::TcpListener;
use std::{env, io::{BufRead, BufReader, Write}, net::TcpStream};

mod req;
mod res;
mod server;
mod util;
mod handler;
mod lib;
// use anyhow::{Error, Ok};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let current_dir = env::current_dir().unwrap();
    println!("Current working directory: {:?}", current_dir);
    server::start_tcp();
}
