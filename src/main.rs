#[allow(unused_imports)]
use std::net::TcpListener;

mod req;
mod res;
mod server;
mod util;
mod handler;
mod lib;

fn main() {
    server::start_tcp();
}
