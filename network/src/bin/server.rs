extern crate futures;
extern crate tokio;

use tokio::{
    prelude::*,
    net::TcpListener,
    codec::Framed,
    io
};

use network::{
    server::handshake,
    MessageCodec,
};

fn main() {
    let addr = "127.0.0.1:33333".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming()
        .map_err(|error| eprintln!("Error receive connection: {:?}", error))
        .for_each(|socket| {
            println!("Connected");

            let client = handshake::new(socket)
                .map_err(|e| {
                    eprintln!("Handshake failed: {:?}", e);

                    io::Error::from(io::ErrorKind::InvalidData)
                })
                .and_then(|socket| {
                    let framed_socket = Framed::new(socket, MessageCodec);
                    framed_socket.for_each(|value| {
                        println!("{}", value);

                        Ok(())
                    })
                })
                .map(|_stream| println!("Done"))
                .map_err(|err| eprintln!("HS Error: {:?}", err));

            tokio::spawn(client);

            Ok(())
        });

    println!("Server running");

    tokio::run(server);
}
