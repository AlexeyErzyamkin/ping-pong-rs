use std::sync::{
    mpsc,
    mpsc::{
        Sender,
        Receiver
    },
    Arc,
    RwLock,
    Mutex
};

use std::collections::HashMap;

use std::thread;

use std::net::{
    TcpListener,
    TcpStream
};

use std::io::Read;

use byteorder::{
    NetworkEndian, ByteOrder
};

use network2::{
    HEADER,
    SocketReader,
    SocketWriter
};

type Message = String;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct PeerId(pub i32);

struct Peer {
    id: PeerId,
    tx: Mutex<Sender<String>>
}

impl Peer {
    pub fn new(id: PeerId, tx: Mutex<Sender<Message>>) -> Self {
        Self {
            id,
            tx
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:43400")?;
    let (tx, rx) = mpsc::channel::<Message>();

    let peers : Arc<RwLock<HashMap<PeerId, Peer>>> = Arc::new(RwLock::new(HashMap::new()));
    let c_peers = peers.clone();

    let send_thread = thread::spawn(move || {
        for msg in rx {
            let r_peers = c_peers.read().unwrap();
            for peer in (*r_peers).values() {
                let peer_tx = peer.tx.lock().unwrap();
                peer_tx.send(msg.clone()).unwrap();
            }
        }
    });

    let mut next_peer_id = 0;

    for client in listener.incoming() {
        match client {
            Ok(stream) => {

                let (send_tx, send_rx) = mpsc::channel();

                let id = PeerId(next_peer_id);
                next_peer_id += 1;

                let peer = Peer::new(id, Mutex::new(send_tx));

                let mut peers = peers.write().unwrap();
                peers.insert(id, peer);

                let local_tx = tx.clone();

                thread::spawn(move || {
                    println!("Connected...");

                    handle(stream, local_tx, send_rx);

                    println!("Disconnected...");
                });
            },
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    send_thread.join().unwrap();

    Ok(())
}

fn handle(mut stream: TcpStream, tx: Sender<Message>, rx: Receiver<Message>) {
    if handshake(&mut stream).is_ok() {
        let send_socket = stream.try_clone().unwrap();
        let send_thread_handle = thread::spawn(move || {
            let mut writer = SocketWriter::new(send_socket);

            for msg in rx {
                writer.write(msg);
            }
        });

        let reader = SocketReader::new(stream);

        for msg in reader {
            match tx.send(msg) {
                Ok(()) => {},
                Err(e) => {
                    eprintln!("Server stopped: {:?}", e);

                    break;
                }
            }
        }

        send_thread_handle.join().unwrap();
    }
}

fn handshake(stream: &mut TcpStream) -> Result<(), ()> {
    let mut buf = [0u8; 2];

    match stream.read_exact(&mut buf) {
        Ok(()) => {
            if NetworkEndian::read_u16(&buf) == HEADER {
                Ok(())
            } else {
                Err(())
            }
        },
        Err(e) => {
            eprintln!("Handshake error: {}", e);

            Err(())
        }
    }
}