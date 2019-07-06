extern crate futures;
extern crate tokio;

use std::thread;

use tokio::{
    prelude::*,
    net::TcpStream,
    io,
    codec::Framed
};

use futures::sync::mpsc;

use network::{
    client::handshake,
    MessageCodec
};

fn main() {
    let addr = "127.0.0.1:33333".parse().unwrap();

    let (tx, rx) = mpsc::channel(100);

    thread::spawn(move || {
        let mut tx = tx;
        loop {
            let mut input = String::new();

            if let Ok(size) = std::io::stdin().read_line(&mut input) {
                if size > 0 {
                    tx = tx
                        .send(input.trim_end().to_string())
                        .wait()
                        .unwrap();
                }
            }
        }
    });

    let connect = TcpStream::connect(&addr);
    let client = handshake::new(connect)
        .map_err(|e| {
            eprintln!("Handshake error: {:?}", e);

            // Separate logic of mapping error
            io::Error::from(io::ErrorKind::InvalidData)
        })
        .and_then(move |socket| {
            let framed = Framed::new(socket, MessageCodec);
            let (reader, writer) = framed.split();

            let sink = rx
                .forward(reader.sink_map_err(|e| eprintln!("{}", e)))
                .map(|_| ());
                // .map_err(|_| io::Error::from(io::ErrorKind::InvalidData));

            // let write = writer
            //     .forward(tx.sink_map_err(|e| eprintln!("{}", e)))
            //     .map_err(|_| io::ErrorKind::InvalidData);

        //    let incoming = writer.for_each(|line| {
        //        println!("{}", line);

        //        Ok(())
        //    });

            tokio::spawn(sink);
            // let read_write = sink.select(incoming);
            // tokio::spawn(read_write);

            Ok(())
        })
        .map(|_| ())
        .map_err(|e| eprintln!("Error: {:?}", e));

    dbg!("About to connect");

    tokio::run(client);

    dbg!("Finished");
}