use std::net::{
    TcpStream
};

use std::{
    str,
    convert::TryFrom,
    io::{Read, Write}
};

use byteorder::{
    NetworkEndian, ByteOrder
};

pub const HEADER : u16 = 0xAEAE;

pub struct SocketReader {
    socket: TcpStream
}

impl SocketReader {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket
        }
    }

    fn read_message_len(&mut self) -> Result<usize, ()> {
        let mut buf = [0u8; 2];

        match self.socket.read_exact(&mut buf) {
            Ok(()) => {
                let len = NetworkEndian::read_u16(&buf) as usize;

                Ok(len)
            },
            Err(e) => {
                eprintln!("Read length error: {}", e);

                Err(())
            }
        }
    }

    fn read_message(&mut self, len: usize) -> Result<String, ()> {
        let mut buf = Vec::new();
        buf.resize(len, 0u8);

        match self.socket.read_exact(&mut buf) {
            Ok(()) => {
                match str::from_utf8(&buf) {
                    Ok(msg) => {
                        Ok(msg.to_string())
                    },
                    Err(e) => {
                        eprintln!("Read message error: {}", e);

                        Err(())
                    }
                }
            },
            Err(e) => {
                eprintln!("Read length error: {}", e);

                Err(())
            }
        }
    }
}

impl Iterator for SocketReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_message_len() {
            Ok(len) => {
                if let Ok(msg) = self.read_message(len) {
                    Some(msg)
                } else {
                    None
                }
            },
            Err(()) => None
        }
    }
}

pub struct SocketWriter {
    socket: TcpStream,
    len_buf: [u8; 2]
}

impl SocketWriter {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            len_buf: [0u8; 2]
        }
    }

    pub fn write(&mut self, msg: String) {
        if let Ok(len) = u16::try_from(msg.len()) {
            NetworkEndian::write_u16(&mut self.len_buf, len);

            self.socket.write_all(&self.len_buf).unwrap();
            self.socket.write_all(msg.as_bytes()).unwrap();
        } 
    }
}

#[cfg(test)]
mod tests {
}
