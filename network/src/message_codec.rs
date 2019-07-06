use std::{
    str,
    convert::TryFrom,
};

use tokio::{
    io,
    codec::{
        Decoder, Encoder,
    },
};

use bytes::{BytesMut, BufMut};

use byteorder::{
    BigEndian, ByteOrder,
};

pub struct MessageCodec;

impl Decoder for MessageCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() > 2 {
            let len = BigEndian::read_u16(&src[..2]) as usize;

            let final_len = (2 + len) as usize;
            if src.len() >= final_len {
                let msg = src.split_to(final_len);

                match str::from_utf8(&msg[2..]) {
                    Ok(line) => return Ok(Some(line.to_string())),
                    Err(err) => {
                        eprintln!("UTF8 Error: {:?}", err);

                        return Err(io::Error::from(io::ErrorKind::InvalidData));
                    }
                }
            }
        }

        Ok(None)
    }
}

impl Encoder for MessageCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len = match u16::try_from(item.len()) {
            Ok(value) => value,
            Err(_) => return Err(io::Error::from(io::ErrorKind::InvalidData))
        };

        let full_len = 2 + item.len();

        dst.reserve(full_len);

        dst.put_u16_be(len);
        dst.put(item);

        Ok(())
    }
}