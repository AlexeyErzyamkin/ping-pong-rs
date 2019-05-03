use tokio::{
    prelude::*,
    net::TcpStream,
    io
};

use futures::future;

use crate::handshake::{HandshakeResult, HandshakeError, HANDSHAKE_LEN, HANDSHAKE_STR, HANDSHAKE_OK, HANDSHAKE_FAIL};

pub fn new(stream: TcpStream) -> impl Future<Item = TcpStream, Error = HandshakeError> {
    io::read_exact(stream, vec![0; HANDSHAKE_LEN])
        .and_then(|(stream, buf)| {
            let (result, response) = if buf == HANDSHAKE_STR.as_bytes() {
                (HandshakeResult::Ok, HANDSHAKE_OK)
            } else {
                (HandshakeResult::Failed, HANDSHAKE_FAIL)
            };

            let next = io::write_all(stream, response)
                .join(future::ok(result));

            next
        })
        .map(|((stream, _), result)| (stream, result))
        .then(|result| {
            let next = match result {
                Ok((stream, hr)) => {
                    match hr {
                        HandshakeResult::Ok => future::ok(stream),
                        HandshakeResult::Failed => future::err(HandshakeError::InvalidData)
                    }
                },
                Err(_) => {
                    future::err(HandshakeError::NetworkError)
                }
            };

            next
        })
}