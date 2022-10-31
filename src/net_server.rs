use std::{
    collections::HashMap,
    io,
    net::{SocketAddr, UdpSocket},
    thread,
};

use gtk::glib::{MainContext, Receiver, PRIORITY_DEFAULT};

fn server_logic(bindings: HashMap<&str, &str>, port: u16) -> io::Result<()> {
    {
        let sock = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], port)))?;

        loop {
            let mut buf = [0u8; 65507];

            let res = sock.recv(&mut buf);

            if let Ok(size) = res {
                let msg = &buf[0..size];
            } else {
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
