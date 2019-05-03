extern crate futures;
extern crate tokio;

use tokio::{
    prelude::*,
    net::{
        TcpStream
    }
};

use network::client::handshake;

fn main() {
    let addr = "127.0.0.1:33333".parse().unwrap();

    let connect = TcpStream::connect(&addr);
    let client = handshake::new(connect)
        .map(|_stream| println!("Handshake OK"))
        .map_err(|e| eprintln!("Handshake ERROR: {:?}", e));

    dbg!("About to connect");

    tokio::run(client);

    dbg!("Finished");
}