//use std::mem;

//use tokio::{
//    prelude::*,
//    net::{TcpStream, tcp::ConnectFuture},
//    io::{WriteAll, ReadExact},
//    io
//};

//use crate::HANDSHAKE_STR;
//use crate::handshake::State::Empty;

pub const HANDSHAKE_STR: &str = "112233";
pub const HANDSHAKE_LEN: usize = 6;

pub const HANDSHAKE_OK: &str = "ok";
pub const HANDSHAKE_FAIL: &str = "er";
pub const HANDSHAKE_RESULT_LEN: usize = 2;

#[derive(Debug)]
pub enum HandshakeResult {
    Ok,
    Failed
}

//pub fn new2(stream: TcpStream) -> impl Future<Item = HandshakeResult, Error = io::Error> {
//    io::write_all(stream, HANDSHAKE_STR)
//        .and_then(|(stream, _)| {
//            io::read_exact(stream, vec![0; HANDSHAKE_RESULT_LEN])
//        })
//        .map(|(stream, buf)| {
//            let result = if buf == HANDSHAKE_OK.as_bytes() {
//                HandshakeResult::Ok
//            } else {
//                HandshakeResult::Failed
//            };
//
//            result
//        })
//}

//pub struct Handshake {
//    state: State
//}

//enum State {
//    Connected(TcpStream),
//    Writing(WriteAll<TcpStream, &'static str>),
//    Reading(ReadExact<TcpStream, [u8; HANDSHAKE_RESULT_LEN]>),
//    Empty
//}

//pub fn new(stream: TcpStream) -> Handshake {
//    Handshake {
//        state: State::Connected(stream)
//    }
//}

//impl Future for Handshake {
//    type Item = HandshakeResult;
//    type Error = io::Error;
//
//    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
//        loop {
//            match mem::replace(&mut self.state, State::Empty) {
//                State::Connected(stream) => {
//                    dbg!("connected");
//
//                    let write = io::write_all(stream, HANDSHAKE_STR);
//
//                    mem::replace(&mut self.state, State::Writing(write));
//                },
//                State::Writing(ref mut write) => {
//                    dbg!("writing");
//
//                    let (stream, buf) = try_ready!(write.poll());
//
//                    dbg!(buf);
//
//                    let mut buf = [0u8; HANDSHAKE_RESULT_LEN];
//                    let read = io::read_exact(stream, buf);
//
//                    mem::replace(&mut self.state, State::Reading(read));
//                },
//                State::Reading(ref mut read) => {
//                    dbg!("reading");
//
//                    let (stream, buf) = try_ready!(read.poll());
//
//                    dbg!(buf);
//
//                    let result = if buf == HANDSHAKE_OK.as_bytes() {
//                        HandshakeResult::Ok
//                    } else {
//                        HandshakeResult::Failed
//                    };
//
//                    dbg!("done");
//
//                    return Ok(Async::Ready(result));
//                },
//                State::Empty => panic!()
//            }
//        }
//    }
//}