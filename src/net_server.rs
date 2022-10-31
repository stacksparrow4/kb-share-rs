use std::{
    collections::HashMap,
    io,
    net::{SocketAddr, UdpSocket},
    thread,
};

use gtk::glib::{MainContext, Receiver, PRIORITY_DEFAULT};

use crate::keycodenames::KEYCODE_NAMES;

fn server_logic(bindings: HashMap<&str, &str>, port: u16) -> io::Result<()> {
    let sock = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], port)))?;

    loop {
        let mut buf = [0u8; 65507];

        match sock.recv_from(&mut buf) {
            Ok((size, from_addr)) => {
                if size == 0 {
                    println!("Ignoring 0 length packet");
                    continue;
                }

                let msg = &buf[..size];

                if msg[0] == 1 {
                    // Request bindings
                    let mut resp = Vec::new();

                    for client_key in bindings.keys() {
                        let keycode = KEYCODE_NAMES.get(client_key).unwrap();

                        let upper_byte = (keycode >> 8) as u8;
                        let lower_byte = (keycode & 0xff) as u8;

                        resp.push(upper_byte);
                        resp.push(lower_byte);
                    }

                    sock.send_to(&resp, from_addr)
                        .expect("Failed to send message back to client");
                }
            }
            Err(_) => {
                println!("Error: sock.recv failed");
            }
        }
    }
}

pub fn start_server_thread(
    bindings: HashMap<&'static str, &'static str>,
    port: u16,
) -> Receiver<String> {
    let (tx, rx) = MainContext::channel(PRIORITY_DEFAULT);

    thread::spawn(move || match server_logic(bindings, port) {
        Err(err) => {
            tx.send(String::from(err.to_string())).unwrap();
        }
        Ok(_) => {}
    });

    return rx;
}
