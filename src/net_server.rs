use std::{collections::HashMap, thread};

use gtk::glib::{MainContext, Receiver, PRIORITY_DEFAULT};

pub fn start_server_thread(bindings: HashMap<&str, &str>) -> Receiver<String> {
    let (tx, rx) = MainContext::channel(PRIORITY_DEFAULT);

    thread::spawn(move || {
        tx.send(String::from("TEST ERROR")).unwrap();
    });

    return rx;
}
