//use std::time::Duration;

use tokio::{
    prelude::*,
    net::{TcpStream, tcp::ConnectFuture},
    io
};

use futures::future;

use crate::handshake::{HandshakeResult, HandshakeError, HANDSHAKE_STR, HANDSHAKE_OK, HANDSHAKE_RESULT_LEN};

pub fn new(connect: ConnectFuture) -> impl Future<Item = TcpStream, Error = HandshakeError> {
    connect
        .and_then(|stream| {
            io::write_all(stream, HANDSHAKE_STR)
        })
        .and_then(|(stream, _)| {
            io::read_exact(stream, vec![0; HANDSHAKE_RESULT_LEN])
        })
        .map(|(stream, buf)| {
            let result = if buf == HANDSHAKE_OK.as_bytes() {
                HandshakeResult::Ok
            } else {
                HandshakeResult::Failed
            };

            (stream, result)
        })
        .then(|result| {
            let next = match result {
                Ok((stream, hr)) => {
                    match hr {
                        HandshakeResult::Ok => future::ok(stream),
                        HandshakeResult::Failed => future::err(HandshakeError::InvalidData)
                    }
                },
                Err(_) => future::err(HandshakeError::NetworkError)
            };

            next
        })
}