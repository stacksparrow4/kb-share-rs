#![windows_subsystem = "windows"]

mod keyboard;
mod keycodenames;
mod menu;
mod net_server;
mod server;

use gtk::gdk::Display;
use gtk::{gio, Application};
use gtk::{prelude::*, CssProvider, StyleContext};

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
