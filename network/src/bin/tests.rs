//extern crate tokio;
//extern crate futures;

use futures::{stream, Future, Stream, Sink};
use futures::future::lazy;
use futures::sync::mpsc;

fn main() {
    tokio::run(lazy(|| {
        let (tx, rx) = mpsc::channel(1_024);

        tokio::spawn({
            stream::iter_ok(0..10).fold(tx, |tx, i| {
                tx.send(format!("Message {} from spawned task", i))
                    .map_err(|e| println!("error = {:?}", e))
            })
                .map(|_| ()) // Drop tx handle
        });

        rx.for_each(|msg| {
            println!("Got `{}`", msg);
            Ok(())
        })
    }));
}