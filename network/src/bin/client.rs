extern crate futures;
extern crate tokio;

use tokio::{
    prelude::*,
    net::TcpStream,
    io,
    codec::Framed,
};

use network::{
    client::handshake,
    MessageCodec
};

fn main() {
    let addr = "127.0.0.1:33333".parse().unwrap();

    let connect = TcpStream::connect(&addr);
    let client = handshake::new(connect)
        .map_err(|e| {
            eprintln!("Handshake error: {:?}", e);

            io::Error::from(io::ErrorKind::InvalidData)
        })
        .and_then(|socket| {
            let framed_socket = Framed::new(socket, MessageCodec);

            framed_socket.send("Hello world".to_string())
        })
        .map(|_| ())
        .map_err(|e| eprintln!("Error: {:?}", e));

    dbg!("About to connect");

    tokio::run(client);

    dbg!("Finished");
}