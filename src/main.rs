extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use self::tokio_proto::TcpServer;
use std::net::SocketAddr;

mod codec;
mod protocol;
mod service;

fn main() {
    let addr = "127.0.0.1:3000".parse::<SocketAddr>().unwrap();
    let server = TcpServer::new(protocol::LineProto, addr);
    
    server.serve(|| Ok(service::EchoService));
}
