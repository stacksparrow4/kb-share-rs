use core::time;
use std::{io, net::UdpSocket, thread};

use gtk::glib::{MainContext, Receiver, Sender, PRIORITY_DEFAULT};
use winapi::um::winuser::GetAsyncKeyState;

use crate::{
    keycodenames::KEYCODE_NAMES,
    util::{bytes_to_u16, u16_to_bytes},
};

const SEND_RATE: u64 = 60;

fn client_logic(
    dest_ip: String,
    dst_port: u16,
    src_port: u16,
    display_msg: Sender<String>,
) -> io::Result<()> {
    let mut src_addr = String::from("127.0.0.1:");
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

    loop {
        // Every tick, get state of all allowed_keycodes and send to server
        let mut packet: Vec<u8> = Vec::new();
        packet.push(2u8);

        for &keycode in allowed_keycodes.iter() {
            let state;
            unsafe {
                state = GetAsyncKeyState(keycode as i32);
            }

            packet.extend(&u16_to_bytes(keycode));
            packet.push(if state == 0 { 0 } else { 1 });
        }

        sock.send_to(&packet, dst_addr)?;

        thread::sleep(time::Duration::from_millis(1000 / SEND_RATE));
    }
}

pub fn start_client_thread(
    dst_ip_str: &str,
    dst_port: u16,
    src_port: u16,
) -> (Receiver<String>, Receiver<String>) {
    let (tx_err, rx_err) = MainContext::channel(PRIORITY_DEFAULT);
    let (tx_msg, rx_msg) = MainContext::channel(PRIORITY_DEFAULT);

    let dst_ip = String::from(dst_ip_str);

    thread::spawn(
        move || match client_logic(dst_ip, dst_port, src_port, tx_msg) {
            Err(err) => {
                tx_err.send(String::from(err.to_string())).unwrap();
            }
            Ok(_) => {}
        },
    );

    return (rx_err, rx_msg);
}
