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

    let client = TcpStream::connect(&addr)
        .and_then(|stream| {
            handshake::new(stream)
        })
        .map(|(_stream, result)| println!("Handshake: {:?}", result))
        .map_err(|e| eprintln!("Error: {:?}", e));

    dbg!("About to connect");

    tokio::run(client);

    dbg!("Finished");
}