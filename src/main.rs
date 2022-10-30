#![windows_subsystem = "windows"]

mod keyboard;
mod keycodenames;
mod menu;
mod server;

use std::rc::Rc;

use gtk::gdk::Display;
use gtk::Entry;
use gtk::{gio, Application, ApplicationWindow};
use gtk::{prelude::*, Box, Button, CssProvider, Label, StyleContext};

use keyboard::presskeydown;
use keycodenames::KEYCODE_NAMES;
use menu::Menu;

fn main() {
    gio::resources_register_include!("kb_share_rs.gresource").expect("Failed to include resources");

    let app = Application::builder()
        .application_id("com.stacksparrow4.KBShareRS")
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let window = Menu::new(app);

    window.present();
}

fn create_key_entry() -> Entry {
    let autocomplete = gtk::ListStore::new(&[String::static_type()]);

    for entry in KEYCODE_NAMES.keys() {
        autocomplete.set(&autocomplete.append(), &[(0, entry)]);
    }

    let key_completion = gtk::EntryCompletion::builder()
        .popup_completion(true)
        .model(&autocomplete)
        .build();
    key_completion.set_text_column(0);

    Entry::builder().completion(&key_completion).build()
}

fn build_server_window(app: &Application) -> ApplicationWindow {
    let current_bindings_label = Label::builder()
        .label("You currently have no bindings")
        .build();

    // Add binding
    let add_binding_client_label = Label::builder().label("Client:").build();
    let add_binding_client_key = create_key_entry();
    let add_binding_server_label = Label::builder().label("Server:").build();
    let add_binding_server_key = create_key_entry();

    let add_binding_bar = Box::builder().build();
    add_binding_bar.append(&add_binding_client_label);
    add_binding_bar.append(&add_binding_client_key);
    add_binding_bar.append(&add_binding_server_label);
    add_binding_bar.append(&add_binding_server_key);

    let add_binding_button = Button::builder().label("Add Binding").build();

    // add_binding_button.connect_clicked(clone!(@weak current_bindings_label, @weak add_binding_client_key, @weak add_binding_server_key =>
    //     move |_| {
    //         let new_text = String::from("Your current bindings are:\n");
    //         new_text
    //     current_bindings_label.set_text(new_text);
    // }));

    // Password
    let password_label = Label::builder().label("Password").build();
    let password_entry = Entry::builder().text("password123").build();

    let password_box = Box::builder().build();
    password_box.append(&password_label);
    password_box.append(&password_entry);

    // Start
    let start_server = Button::builder().label("START").build();

    start_server.connect_clicked(|_| {
        presskeydown();
    });

    // Window
    let window_box = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    window_box.append(&current_bindings_label);
    window_box.append(&add_binding_bar);
    window_box.append(&add_binding_button);
    window_box.append(&password_box);
    window_box.append(&start_server);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("KB Share Server")
        .child(&window_box)
        .build();

    return window;
}
