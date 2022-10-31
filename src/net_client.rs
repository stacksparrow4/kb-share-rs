use core::time;
use std::{collections::HashMap, io, net::UdpSocket, sync::mpsc, thread};

use gtk::glib::{MainContext, Receiver, Sender, PRIORITY_DEFAULT};

use crate::{
    keycodenames::KEYCODE_NAMES,
    util::{bytes_to_u16, u16_to_bytes},
};

const SEND_RATE: u64 = 60;

#[derive(Debug)]
pub struct KeyPressMsg {
    pub keycode: u16,
    pub is_pressed: bool,
}

fn client_logic(
    dest_ip: String,
    dst_port: u16,
    src_port: u16,
    display_msg: Sender<String>,
    read_key: mpsc::Receiver<KeyPressMsg>,
) -> io::Result<()> {
    let mut src_addr = String::from("0.0.0.0:");
    src_addr.push_str(src_port.to_string().as_str());

    let sock = UdpSocket::bind(src_addr)?;

    let mut dst_addr = dest_ip;
    dst_addr.push_str(":");
    dst_addr.push_str(dst_port.to_string().as_str());

    let dst_addr = dst_addr.as_str();

    // Request the keycodes to listen for
    sock.send_to(&[1u8], dst_addr)?;

    // Recieve and display keycodes
    let mut buf = [0u8; 65507];
    let size = sock.recv(&mut buf)?;

    if size % 2 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Recieved invalid data from server",
        ));
    }

    let msg = &buf[..size];

    let mut allowed_keycodes: Vec<u16> = Vec::new();

    for i in 0..(size / 2) {
        let original = bytes_to_u16(&msg[(2 * i)..(2 * i + 2)]);

        if KEYCODE_NAMES.values().find(|&&x| x == original).is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Recieved invalid keycode from server",
            ));
        }

        allowed_keycodes.push(original);
    }

    let keycode_names: Vec<String> = allowed_keycodes
        .iter()
        .map(|&x| String::from(*KEYCODE_NAMES.entries().find(|&y| y.1 == &x).unwrap().0))
        .collect();

    let allowed_keys_str = format!("Allowed keys: {}", keycode_names.join(", "));
    display_msg.send(allowed_keys_str).unwrap();

    let mut curr_states: HashMap<u16, bool> = HashMap::new();
    for curr_keycode in allowed_keycodes {
        curr_states.insert(curr_keycode, false);
    }

    loop {
        // Event loop
        loop {
            match read_key.try_recv() {
                Ok(msg) => {
                    if curr_states.contains_key(&msg.keycode) {
                        curr_states.insert(msg.keycode, msg.is_pressed);
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }

        // Send state to server
        let mut packet: Vec<u8> = Vec::new();
        packet.push(2u8);

        for (keycode, pressed) in curr_states.iter() {
            packet.extend(&u16_to_bytes(*keycode));
            packet.push(if *pressed { 1 } else { 0 });
        }

        sock.send_to(&packet, dst_addr)?;

        thread::sleep(time::Duration::from_millis(1000 / SEND_RATE));
    }
}

pub fn start_client_thread(
    dst_ip_str: &str,
    dst_port: u16,
    src_port: u16,
) -> (
    Receiver<String>,
    Receiver<String>,
    mpsc::Sender<KeyPressMsg>,
) {
    let (tx_err, rx_err) = MainContext::channel(PRIORITY_DEFAULT);
    let (tx_msg, rx_msg) = MainContext::channel(PRIORITY_DEFAULT);
    let (tx_key, rx_key) = mpsc::channel();

    let dst_ip = String::from(dst_ip_str);

    thread::spawn(
        move || match client_logic(dst_ip, dst_port, src_port, tx_msg, rx_key) {
            Err(err) => {
                tx_err.send(String::from(err.to_string())).unwrap();
            }
            Ok(_) => {}
        },
    );

    return (rx_err, rx_msg, tx_key);
}
