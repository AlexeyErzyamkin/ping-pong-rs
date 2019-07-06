use byteorder::{
    NetworkEndian, ByteOrder
};

use std::{
    // cell::RefCell,
    thread,
    sync::{
        mpsc,
        // Arc
    },
    net::TcpStream,
    // convert::TryFrom,
    io::Write
};

use network2::{
    HEADER,
    SocketReader,
    SocketWriter
};

fn main() -> std::io::Result<()> {
    
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            let mut input = String::new();

            if let Ok(size) = std::io::stdin().read_line(&mut input) {
                if size > 0 {
                    tx.send(input.trim_end().to_string()).unwrap();
                }
            }
        }
    });

    let mut socket = TcpStream::connect("127.0.0.1:43400")?;
    let rx_socket = socket.try_clone()?;

    let mut buf = [0u8; 2];
    NetworkEndian::write_u16(&mut buf, HEADER);

    socket.write_all(&buf).unwrap();
    socket.flush().unwrap();

    thread::spawn(move || {
        let reader = SocketReader::new(rx_socket);
        for msg in reader {
            println!("Msg: {}", msg);
        }
    });
    
    let mut writer = SocketWriter::new(socket);

    for input in rx {
        writer.write(input);
    }

    Ok(())
}