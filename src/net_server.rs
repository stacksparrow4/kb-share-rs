use std::{
    collections::HashMap,
    io,
    net::{SocketAddr, UdpSocket},
    thread,
};

use enigo::{Enigo, KeyboardControllable};
use gtk::glib::{MainContext, Receiver, PRIORITY_DEFAULT};

use crate::{
    keycodenames::KEYCODE_NAMES,
    util::{bytes_to_u16, u16_to_bytes},
};

fn server_logic(bindings: HashMap<&str, &str>, port: u16) -> io::Result<()> {
    let mut enigo = Enigo::new();

    let sock = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], port)))?;

    let mut key_states: HashMap<u16, bool> = HashMap::new();
    for key_str in bindings.values() {
        let curr_keycode = *KEYCODE_NAMES.get(key_str).unwrap();
        println!("Adding keycode {}", curr_keycode);
        key_states.insert(curr_keycode, false);
    }

    let mut keycode_bindings: HashMap<u16, u16> = HashMap::new();
    for (k, v) in bindings.iter() {
        keycode_bindings.insert(
            *KEYCODE_NAMES.get(k).unwrap(),
            *KEYCODE_NAMES.get(v).unwrap(),
        );
    }

    loop {
        let mut buf = [0u8; 65507];

        match sock.recv_from(&mut buf) {
            Ok((size, from_addr)) => {
                if size == 0 {
                    println!("Ignoring 0 length packet");
                    continue;
                }

                let msg = &buf[..size];

                match msg[0] {
                    1 => {
                        // Request bindings
                        let mut resp = Vec::new();

                        for client_key in bindings.keys() {
                            let keycode = KEYCODE_NAMES.get(client_key).unwrap();

                            resp.extend(u16_to_bytes(*keycode));
                        }

                        sock.send_to(&resp, from_addr)
                            .expect("Failed to send message back to client");
                    }
                    2 => {
                        // Set key states
                        // Packet format:
                        // [keycode_upper, keycode_lower, state]
                        // where state = 0 = up
                        // state = 1 = down

                        let payload = &msg[1..];

                        if payload.len() % 3 != 0 {
                            println!("Ignoring invalid packet!");
                            continue;
                        }

                        for i in 0..(payload.len() / 3) {
                            let client_keycode = bytes_to_u16(&payload[(i * 3)..(i * 3 + 2)]);

                            let keycode = keycode_bindings.get(&client_keycode);

                            if let None = keycode {
                                println!("Unmapped keycode {}", client_keycode);
                                continue;
                            }

                            let keycode = *keycode.unwrap();

                            let new_state = payload[i * 3 + 2] == 1;

                            match key_states.get(&keycode) {
                                Some(pressed) => {
                                    if *pressed != new_state {
                                        if new_state {
                                            enigo.key_down(enigo::Key::Raw(keycode));
                                        } else {
                                            enigo.key_up(enigo::Key::Raw(keycode));
                                        }

                                        key_states.insert(keycode, new_state);
                                    }
                                }
                                None => {
                                    println!("Ignoring invalid keycode");
                                }
                            }
                        }
                    }
                    cmd => {
                        println!("Unknown command: {}", cmd);
                    }
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
