extern crate futures;
extern crate tokio;

use tokio::{
    prelude::*,
    net::TcpListener,
};

use network::{
    server::handshake
};

fn main() {
    let addr = "127.0.0.1:33333".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming()
        .map_err(|error| eprintln!("Error receive connection: {:?}", error))
        .for_each(|stream| {
            println!("Connected");

            let client = handshake::new(stream)
                .map(|_stream| println!("Done"))
                .map_err(|err| eprintln!("HS Error: {:?}", err));

            tokio::spawn(client);

            Ok(())
        });

    println!("Server running");

    tokio::run(server);
}
